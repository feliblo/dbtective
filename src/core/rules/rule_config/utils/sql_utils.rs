/// Strip SQL comments from code in a single pass.
///
/// Removes both single-line (`--`) and multi-line (`/* ... */`) comments.
/// Respects string literals (single-quoted) so `--` or `/*` inside strings are preserved.
pub fn strip_sql_comments(code: &str) -> String {
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(len);
    let mut i = 0;

    while i < len {
        // Check for single-quoted string literal — skip through it verbatim
        if bytes[i] == b'\'' {
            result.push('\'');
            i += 1;
            while i < len {
                if bytes[i] == b'\'' {
                    result.push('\'');
                    i += 1;
                    // Handle escaped quotes ('')
                    if i < len && bytes[i] == b'\'' {
                        result.push('\'');
                        i += 1;
                        continue;
                    }
                    break;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Check for block comment /*...*/
        if i + 1 < len && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < len {
                if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            // Handle unclosed block comment — skip to end
            if i + 1 >= len && !(i >= 2 && bytes[i - 2] == b'*' && bytes[i - 1] == b'/') {
                break;
            }
            continue;
        }

        // Check for line comment --
        if i + 1 < len && bytes[i] == b'-' && bytes[i + 1] == b'-' {
            // Skip until end of line
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        result.push(bytes[i] as char);
        i += 1;
    }

    result
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

    #[test]
    fn test_string_literal_with_dash_preserved() {
        let code = "SELECT '--not a comment' FROM table";
        let stripped = strip_sql_comments(code);
        assert_eq!(stripped, code);
    }

    #[test]
    fn test_string_literal_with_block_comment_preserved() {
        let code = "SELECT '/* not a comment */' FROM table";
        let stripped = strip_sql_comments(code);
        assert_eq!(stripped, code);
    }

    #[test]
    fn test_escaped_single_quote_in_string() {
        let code = "SELECT 'it''s -- fine' FROM table";
        let stripped = strip_sql_comments(code);
        assert_eq!(stripped, code);
    }

    #[test]
    fn test_multiple_block_comments() {
        let code = "SELECT /* xxx */ id /* yyy */ FROM tbl";
        let stripped = strip_sql_comments(code);
        assert!(stripped.contains("SELECT "));
        assert!(stripped.contains(" id "));
        assert!(stripped.contains(" FROM tbl"));
        assert!(!stripped.contains("xxx"));
        assert!(!stripped.contains("yyy"));
    }

    #[test]
    fn test_mixed_comments() {
        let code = "SELECT id -- line comment\n/* block */ FROM table";
        let stripped = strip_sql_comments(code);
        assert!(stripped.contains("SELECT id "));
        assert!(stripped.contains(" FROM table"));
        assert!(!stripped.contains("line comment"));
        assert!(!stripped.contains("block"));
    }
}
