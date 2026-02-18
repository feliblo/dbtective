/// Strip SQL comments from code.
/// Removes both single-line (`--`) and multi-line (`/* ... */`) comments.
pub fn strip_sql_comments(code: &str) -> String {
    let mut result = code.to_string();
    while let Some(start) = result.find("/*") {
        if let Some(end) = result[start..].find("*/") {
            result.replace_range(start..start + end + 2, "");
        } else {
            result.replace_range(start.., "");
        }
    }

    result
        .lines()
        .map(|line| line.split_once("--").map_or(line, |(before, _)| before))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_single_line_comment() {
        let code = "SELECT * FROM table -- this is a comment\nWHERE 1=1";
        let stripped = strip_sql_comments(code);
        assert!(stripped.contains("SELECT * FROM table "));
        assert!(!stripped.contains("this is a comment"));
    }

    #[test]
    fn test_strip_multi_line_comment() {
        let code = "SELECT * /* some comment */ FROM table";
        let stripped = strip_sql_comments(code);
        assert!(!stripped.contains("some comment"));
        assert!(stripped.contains("FROM table"));
    }

    #[test]
    fn test_strip_entire_line_comment() {
        let code = "-- entire line comment\nSELECT 1";
        let stripped = strip_sql_comments(code);
        assert!(!stripped.contains("entire line comment"));
        assert!(stripped.contains("SELECT 1"));
    }

    #[test]
    fn test_strip_nested_multi_line_comment() {
        let code = "/* \n  source('raw', 'table') \n  ref('model') \n*/ SELECT 1";
        let stripped = strip_sql_comments(code);
        assert!(!stripped.contains("source("));
        assert!(!stripped.contains("ref("));
        assert!(stripped.contains("SELECT 1"));
    }

    #[test]
    fn test_no_comments() {
        let code = "SELECT id, name FROM users";
        let stripped = strip_sql_comments(code);
        assert_eq!(stripped, code);
    }

    #[test]
    fn test_unclosed_block_comment() {
        let code = "SELECT 1 /* unclosed comment";
        let stripped = strip_sql_comments(code);
        assert!(!stripped.contains("unclosed comment"));
    }
}
