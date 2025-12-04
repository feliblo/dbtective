### Changelog

All notable changes to this project will be documented in this file. Dates are displayed in UTC.

## v0.1.10 (2025-12-04)

### Feat

- **checks**: add all_columns_are_documented for nodes

### Fix

- **checks**: fix applies_to for columns_are_documented and clippy warnings

### Refactor

- **checks**: change filename to other_manifest_object_checks.rs
- **catalog**: preparation for catalog based tests
- **run**: refactor run to use unwrap_or_exit helper

#### [v0.1.9](https://github.com/feliblo/dbtective/compare/v0.1.8...v0.1.9)

> 2 December 2025

- Docs/update config docs [`#37`](https://github.com/feliblo/dbtective/pull/37)
- feat(cfg): add catalog tests to config [`e0bc29c`](https://github.com/feliblo/dbtective/commit/e0bc29cb6b0f9f725c07ff0db649188f276fc967)
- feat(checks): run checks on all (data-related) manifest objects [`67c5ebe`](https://github.com/feliblo/dbtective/commit/67c5ebe9e6165313f409eb5502317f2bc2499d0b)
- test(cfg): add config parsing tests [`5b2b13f`](https://github.com/feliblo/dbtective/commit/5b2b13f197f754ffb50f0888991c97c1394c767e)

#### [v0.1.8](https://github.com/feliblo/dbtective/compare/v0.1.7...v0.1.8)

> 30 November 2025

- Docs/update config docs [`#37`](https://github.com/feliblo/dbtective/pull/37)
- bump: version 0.1.6 → 0.1.7 [`89e4a0a`](https://github.com/feliblo/dbtective/commit/89e4a0abf45eaab1efc05a54220b63104809f6b5)
- bump: version 0.1.7 → 0.1.8 [`c83be05`](https://github.com/feliblo/dbtective/commit/c83be05f8d8a990f06f71bdff9cb5a9a3110f8e0)

#### [v0.1.7](https://github.com/feliblo/dbtective/compare/v0.1.6...v0.1.7)

> 30 November 2025

- bump: version 0.1.5 → 0.1.6 [`c8478cd`](https://github.com/feliblo/dbtective/commit/c8478cd67387a80727ee57554287063fed982a63)
- bump: version 0.1.6 → 0.1.7 [`794ade5`](https://github.com/feliblo/dbtective/commit/794ade55fd5d6e378a3dc8a592309f2982d47bd8)
- docs(checks): improve has_description example [`587e7e6`](https://github.com/feliblo/dbtective/commit/587e7e6c7c0684c9ecd904cf1afb4d258c05a19d)

#### [v0.1.6](https://github.com/feliblo/dbtective/compare/v0.1.5...v0.1.6)

> 30 November 2025

- bump: version 0.1.5 → 0.1.6 [`b05535a`](https://github.com/feliblo/dbtective/commit/b05535a5938a62d363f3d3f8ec2c473d7760def0)

#### [v0.1.5](https://github.com/feliblo/dbtective/compare/v0.1.4...v0.1.5)

> 30 November 2025

- bump: version 0.1.4 → 0.1.5 [`aca87b2`](https://github.com/feliblo/dbtective/commit/aca87b268e34c1ec00bc4b177ce16d0a4b33aa3f)
- bump: version 0.1.3 → 0.1.4 [`54ab758`](https://github.com/feliblo/dbtective/commit/54ab7583b06b46587c21ed258d550d0ad293bab3)

#### [v0.1.4](https://github.com/feliblo/dbtective/compare/v0.1.3...v0.1.4)

> 30 November 2025

- feat(pypi): add pypi release pipeline [`#32`](https://github.com/feliblo/dbtective/pull/32)
- bump: version 0.1.2 → 0.1.3 [`614a139`](https://github.com/feliblo/dbtective/commit/614a1394b35edac6963425a0e211b7d108d38d7c)
- bump: version 0.1.3 → 0.1.4 [`a38cd76`](https://github.com/feliblo/dbtective/commit/a38cd76fe75e7d6971def3dc7d77dbbf81a420c5)

#### [v0.1.3](https://github.com/feliblo/dbtective/compare/v0.1.2...v0.1.3)

> 30 November 2025

- bump: version 0.1.2 → 0.1.3 [`7a4ecb5`](https://github.com/feliblo/dbtective/commit/7a4ecb5d0ac1241bb3ab3ca95d6fd5f26bf55196)
- bump: version 0.1.1 → 0.1.2 [`8ecec19`](https://github.com/feliblo/dbtective/commit/8ecec19e7ba2d4eccf75c5a233d11d64e6c3f5a6)
- fix(ci): enable homebrew publishing [`cff763b`](https://github.com/feliblo/dbtective/commit/cff763bbd76c90a36219d3d2d72ec7fe6bc829cc)

#### [v0.1.2](https://github.com/feliblo/dbtective/compare/v0.1.0-alpha...v0.1.2)

> 30 November 2025

- refactor(anyhow): propagate errors & impove/introduce (integration) testing [`ef9289a`](https://github.com/feliblo/dbtective/commit/ef9289a40b500286e887a96d8cda539e012a336d)
- refactor(AppliesTo): change appliesto to work on all manifest objects [`8aa2e67`](https://github.com/feliblo/dbtective/commit/8aa2e67c4c7231a640aa4d86ca52e953a7a2ab5f)
- refactor(release): use cargo-dist for release pipeline [`ec2a269`](https://github.com/feliblo/dbtective/commit/ec2a269e3ac4bfd113595f70d2425bace7bf4539)

#### v0.1.0-alpha

> 25 November 2025

- 🎉 [init] Create dbtective project [`a3f50d4`](https://github.com/feliblo/dbtective/commit/a3f50d4f3b2e7a76c5fa4c4a37c87ea6f8bed876)
- 🚧 [serde] Model manifest in progress [`eab955c`](https://github.com/feliblo/dbtective/commit/eab955c2839da172e5a4d4981089bb17ec4d38d4)
- 🚧 [config] Initialize ManifestConfig parsing [`4f56ddf`](https://github.com/feliblo/dbtective/commit/4f56ddf7368e41019ed41aba6b41ef7ad6cc66fe)
