use crate::core::catalog::Catalog;
use crate::core::manifest::Manifest;
use anyhow::{anyhow, Result};
use clap::ValueEnum;
use log::debug;
use owo_colors::OwoColorize;
use std::path::{Path, PathBuf};

#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ArtifactFormat {
    #[default]
    Auto,
    Json,
    Parquet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactSource {
    Json,
    Parquet,
}

impl ArtifactSource {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Json => "JSON artifacts",
            Self::Parquet => "Parquet index",
        }
    }
}

pub struct ArtifactPaths {
    pub manifest: PathBuf,
    pub catalog: PathBuf,
    pub index_dir: PathBuf,
}

impl ArtifactPaths {
    pub fn new(
        entry_point: &str,
        manifest_file: &str,
        catalog_file: &str,
        index_dir: &str,
    ) -> Self {
        let root = Path::new(entry_point);
        Self {
            manifest: root.join(manifest_file),
            catalog: root.join(catalog_file),
            index_dir: root.join(index_dir),
        }
    }

    fn has_json(&self) -> bool {
        self.manifest.is_file()
    }

    fn has_index(&self) -> bool {
        self.index_dir.is_dir()
            && std::fs::read_dir(&self.index_dir).is_ok_and(|mut entries| {
                entries.any(|e| {
                    e.is_ok_and(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                })
            })
    }
}

/// dbt v2 writes both formats, so a tie must resolve to Parquet or the index
/// would never be used. JSON only wins when it is from a strictly newer dbt.
///
/// # Errors
/// If an explicitly requested format is not present.
pub fn resolve(paths: &ArtifactPaths, format: ArtifactFormat) -> Result<ArtifactSource> {
    match format {
        ArtifactFormat::Json => {
            if !paths.has_json() {
                return Err(anyhow!(
                    "--artifact-format json was set but no manifest was found at {}",
                    paths.manifest.display()
                ));
            }
            Ok(ArtifactSource::Json)
        }
        ArtifactFormat::Parquet => {
            if !paths.has_index() {
                return Err(anyhow!(
                    "--artifact-format parquet was set but no Parquet index was found at {}.\n\
                     Generate one with a dbt v2 command such as 'dbt build --write-index'.",
                    paths.index_dir.display()
                ));
            }
            Ok(ArtifactSource::Parquet)
        }
        ArtifactFormat::Auto => Ok(auto_detect(paths)),
    }
}

fn auto_detect(paths: &ArtifactPaths) -> ArtifactSource {
    match (paths.has_json(), paths.has_index()) {
        (false, true) => ArtifactSource::Parquet,
        (true | false, false) => ArtifactSource::Json,
        (true, true) => {
            let json = Manifest::peek_dbt_version(&paths.manifest).ok();
            let parquet = Manifest::peek_index_dbt_version(&paths.index_dir)
                .ok()
                .flatten();
            debug!("artifact versions: json={json:?} parquet={parquet:?}");

            match (json.as_deref(), parquet.as_deref()) {
                (Some(j), Some(p)) if is_newer(j, p) => ArtifactSource::Json,
                _ => ArtifactSource::Parquet,
            }
        }
    }
}

/// Compares dotted versions numerically, ignoring any pre-release suffix.
fn is_newer(candidate: &str, than: &str) -> bool {
    numeric_parts(candidate) > numeric_parts(than)
}

fn numeric_parts(version: &str) -> Vec<u64> {
    version
        .split(['-', '+'])
        .next()
        .unwrap_or_default()
        .split('.')
        .map(|p| p.parse::<u64>().unwrap_or_default())
        .collect()
}

/// # Errors
/// If the chosen artifacts cannot be read.
pub fn load_manifest(paths: &ArtifactPaths, source: ArtifactSource) -> Result<Manifest> {
    match source {
        ArtifactSource::Json => Manifest::from_file(&paths.manifest),
        ArtifactSource::Parquet => {
            Manifest::from_index_with_json(&paths.index_dir, Some(&paths.manifest))
        }
    }
}

/// # Errors
/// If the chosen artifacts cannot be read.
pub fn load_catalog(paths: &ArtifactPaths, source: ArtifactSource) -> Result<Catalog> {
    match source {
        ArtifactSource::Json => Catalog::from_file(&paths.catalog),
        // The index records far fewer warehouse columns than catalog.json, so
        // prefer the catalog when dbt wrote one (`--write-catalog`).
        ArtifactSource::Parquet if paths.catalog.is_file() => Catalog::from_file(&paths.catalog),
        ArtifactSource::Parquet => {
            print_sparse_index_catalog_warning();
            Catalog::from_index(&paths.index_dir)
        }
    }
}

/// Without a catalog the index's own columns are used, and dbt v2 records only a
/// fraction of the warehouse columns there. Warn rather than silently under-report.
fn print_sparse_index_catalog_warning() {
    eprintln!(
        "{} No catalog.json found next to the Parquet index.",
        "⚠".yellow().bold()
    );
    eprintln!(
        "  {}",
        "Column rules will use the index, which records far fewer warehouse columns.".yellow()
    );
    eprintln!(
        "  {}",
        "Generate one with `dbt compile --write-catalog` for full coverage.".cyan()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use dbt_artifact_parser::parquet::test_writer::{Cell, ColumnKind, IndexBuilder, TableBuilder};
    use std::io::Write;
    use tempfile::TempDir;

    /// A dbt project directory with whichever artifacts the test needs.
    struct Project {
        dir: TempDir,
    }

    impl Project {
        fn new() -> Self {
            let dir = TempDir::new().unwrap();
            std::fs::create_dir_all(dir.path().join("target")).unwrap();
            Self { dir }
        }

        /// dbt writes manifest.json on every invocation, v1 and v2 alike.
        fn with_manifest_json(self, dbt_version: &str) -> Self {
            let path = self.dir.path().join("target/manifest.json");
            let mut file = std::fs::File::create(path).unwrap();
            write!(
                file,
                r#"{{"metadata":{{"dbt_schema_version":"https://schemas.getdbt.com/dbt/manifest/v12.json","dbt_version":"{dbt_version}","project_name":"my_project"}},"nodes":{{}},"sources":{{}}}}"#
            )
            .unwrap();
            self
        }

        fn with_catalog_json(self) -> Self {
            let path = self.dir.path().join("target/catalog.json");
            let mut file = std::fs::File::create(path).unwrap();
            write!(
                file,
                r#"{{"metadata":{{"dbt_schema_version":"https://schemas.getdbt.com/dbt/catalog/v1.json","dbt_version":"2.0.0-beta.1","generated_at":"2026-08-15T00:00:00Z","env":{{}}}},"nodes":{{}},"sources":{{}},"errors":null}}"#
            )
            .unwrap();
            self
        }

        /// `dbt build --write-index` output, reduced to what resolution reads.
        fn with_index(self, dbt_version: &str) -> Self {
            let project = TableBuilder::new(&[
                ("project_name", ColumnKind::Utf8),
                ("dbt_version", ColumnKind::Utf8),
            ])
            .row(&[
                Cell::Str("my_project"),
                Cell::Owned(dbt_version.to_string()),
            ]);
            let nodes = TableBuilder::new(&[
                ("unique_id", ColumnKind::Utf8),
                ("name", ColumnKind::Utf8),
                ("resource_type", ColumnKind::Utf8),
                ("package_name", ColumnKind::Utf8),
            ])
            .row(&[
                Cell::Str("model.my_project.orders"),
                Cell::Str("orders"),
                Cell::Str("model"),
                Cell::Str("my_project"),
            ]);
            IndexBuilder::new(&self.dir.path().join("target/index"))
                .unwrap()
                .table("dbt.project", project)
                .unwrap()
                .table("dbt.nodes", nodes)
                .unwrap();
            self
        }

        fn paths(&self) -> ArtifactPaths {
            ArtifactPaths::new(
                self.dir.path().to_str().unwrap(),
                "target/manifest.json",
                "target/catalog.json",
                "target/index",
            )
        }
    }

    #[test]
    fn a_dbt_v1_project_uses_json() {
        let project = Project::new().with_manifest_json("1.10.2");
        let source = resolve(&project.paths(), ArtifactFormat::Auto).unwrap();
        assert_eq!(source, ArtifactSource::Json);
    }

    /// dbt v2 writes both formats, so a tie has to resolve to Parquet or the
    /// index would never be used.
    #[test]
    fn a_dbt_v2_project_uses_the_index_even_though_json_is_present() {
        let project = Project::new()
            .with_manifest_json("2.0.0-beta.1")
            .with_index("2.0.0-beta.1");
        let source = resolve(&project.paths(), ArtifactFormat::Auto).unwrap();
        assert_eq!(source, ArtifactSource::Parquet);
    }

    /// Someone ran dbt v2 once, then went back to v1: the fresher JSON wins.
    #[test]
    fn a_newer_json_manifest_beats_a_stale_index() {
        let project = Project::new()
            .with_manifest_json("2.1.0")
            .with_index("2.0.0-beta.1");
        let source = resolve(&project.paths(), ArtifactFormat::Auto).unwrap();
        assert_eq!(source, ArtifactSource::Json);
    }

    #[test]
    fn an_index_without_json_is_used_on_its_own() {
        let project = Project::new().with_index("2.0.0-beta.1");
        let source = resolve(&project.paths(), ArtifactFormat::Auto).unwrap();
        assert_eq!(source, ArtifactSource::Parquet);
    }

    /// With nothing to read, resolution stays on JSON so the loader can report
    /// the familiar "unable to open manifest" error.
    #[test]
    fn an_empty_target_falls_through_to_json() {
        let project = Project::new();
        let source = resolve(&project.paths(), ArtifactFormat::Auto).unwrap();
        assert_eq!(source, ArtifactSource::Json);
    }

    #[test]
    fn an_explicit_format_is_obeyed() {
        let project = Project::new()
            .with_manifest_json("2.0.0-beta.1")
            .with_index("2.0.0-beta.1");
        assert_eq!(
            resolve(&project.paths(), ArtifactFormat::Json).unwrap(),
            ArtifactSource::Json
        );
        assert_eq!(
            resolve(&project.paths(), ArtifactFormat::Parquet).unwrap(),
            ArtifactSource::Parquet
        );
    }

    #[test]
    fn asking_for_parquet_without_an_index_explains_how_to_make_one() {
        let project = Project::new().with_manifest_json("1.10.2");
        let err = resolve(&project.paths(), ArtifactFormat::Parquet)
            .unwrap_err()
            .to_string();
        assert!(err.contains("--write-index"), "unexpected error: {err}");
    }

    #[test]
    fn asking_for_json_without_a_manifest_is_an_error() {
        let project = Project::new().with_index("2.0.0-beta.1");
        let err = resolve(&project.paths(), ArtifactFormat::Json)
            .unwrap_err()
            .to_string();
        assert!(err.contains("no manifest"), "unexpected error: {err}");
    }

    #[test]
    fn loads_a_manifest_from_the_index() {
        let project = Project::new().with_index("2.0.0-beta.1");
        let manifest = load_manifest(&project.paths(), ArtifactSource::Parquet).unwrap();
        assert!(manifest.nodes.contains_key("model.my_project.orders"));
    }

    /// The index records far fewer warehouse columns, so a catalog.json written
    /// by `--write-catalog` is preferred when it exists.
    #[test]
    fn the_parquet_path_prefers_a_real_catalog() {
        let project = Project::new()
            .with_index("2.0.0-beta.1")
            .with_catalog_json();
        let catalog = load_catalog(&project.paths(), ArtifactSource::Parquet).unwrap();
        assert_eq!(catalog.metadata.dbt_version, "2.0.0-beta.1");
    }

    #[test]
    fn the_parquet_path_falls_back_to_the_index_catalog() {
        let project = Project::new().with_index("2.0.0-beta.1");
        let catalog = load_catalog(&project.paths(), ArtifactSource::Parquet).unwrap();
        assert!(
            catalog.nodes.is_empty(),
            "nothing was built, so nothing is catalogued"
        );
    }

    #[test]
    fn newer_compares_numerically() {
        assert!(is_newer("2.0.0", "1.11.6"));
        assert!(is_newer("1.11.10", "1.11.9"));
        assert!(!is_newer("1.11.6", "2.0.0"));
        assert!(!is_newer("2.0.0", "2.0.0"));
    }

    #[test]
    fn prerelease_suffix_is_ignored() {
        assert!(!is_newer("2.0.0", "2.0.0-beta.1"));
        assert!(is_newer("2.1.0", "2.0.0-beta.1"));
    }
}
