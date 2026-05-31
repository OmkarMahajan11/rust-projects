use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    StringToken(String),
    NumberToken(f64),
    True,
    False,
    Null,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let bytes = input.as_bytes();

    let mut res = Vec::new();
    let mut idx: usize = 0;

    while idx < input.len() {
        match bytes[idx] {
            b'{' => res.push(Token::LeftBrace),
            b'}' => res.push(Token::RightBrace),
            b'[' => res.push(Token::LeftBracket),
            b']' => res.push(Token::RightBracket),
            b':' => res.push(Token::Colon),
            b',' => res.push(Token::Comma),
            b't' if input[idx..].starts_with("true") => {
                res.push(Token::True);
                idx += 3;
            }
            b'f' if input[idx..].starts_with("false") => {
                res.push(Token::False);
                idx += 4;
            }
            b'n' if input[idx..].starts_with("null") => {
                res.push(Token::Null);
                idx += 3;
            }
            b'0'..=b'9' | b'-' => {
                let start = idx;
                let mut has_dot = false;

                if input.as_bytes()[idx] == b'-' {
                    idx += 1;
                }

                while idx < input.len() {
                    let b = input.as_bytes()[idx];
                    match b {
                        b'0'..=b'9' => idx += 1,
                        b'.' if has_dot => anyhow::bail!("invalid number: multiple dots"),
                        b'.' => {
                            has_dot = true;
                            idx += 1;
                        }
                        _ => break,
                    }
                }

                let num_str = &input[start..idx];
                let number: f64 = num_str.parse()?;
                res.push(Token::NumberToken(number));
                // idx is already past the last numeric char, we don't want to increment again
                continue;
            }
            b'"' => {
                // skip the current "
                idx += 1;
                let start = idx;
                while idx < input.len() {
                    let b = input.as_bytes()[idx];
                    match b {
                        b'\\' => idx += 2,
                        b'"' => break,
                        _ => idx += 1,
                    }
                }

                if idx >= input.len() {
                    anyhow::bail!("unterminated string");
                }

                // char at idx is ", we don't need to include it
                let content = &input[start..idx];
                res.push(Token::StringToken(content.to_string()));
            }
            b' ' | b'\t' | b'\n' | b'\r' => {
                idx += 1;
                continue;
            }
            _ => anyhow::bail!("unexpected character: '{}'", bytes[idx] as char),
        }
        idx += 1;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object() {
        let tokens = tokenize(r#"{"name": "alice", "age": 30, "active": true}"#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::StringToken("name".into()),
                Token::Colon,
                Token::StringToken("alice".into()),
                Token::Comma,
                Token::StringToken("age".into()),
                Token::Colon,
                Token::NumberToken(30.0),
                Token::Comma,
                Token::StringToken("active".into()),
                Token::Colon,
                Token::True,
                Token::RightBrace,
            ]
        );
    }

    #[test]
    fn test_emoji() {
        let tokens = tokenize(r#"{"emoji": "hello 🌍"}"#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::StringToken("emoji".into()),
                Token::Colon,
                Token::StringToken("hello 🌍".into()),
                Token::RightBrace,
            ]
        );
    }

    #[test]
    fn test_escaped_quotes() {
        let tokens = tokenize(r#"{"key": "say \"hi\""}"#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::StringToken("key".into()),
                Token::Colon,
                Token::StringToken("say \\\"hi\\\"".into()),
                Token::RightBrace,
            ]
        );
    }

    #[test]
    fn test_array_and_keywords() {
        let tokens = tokenize(r#"[true, false, null]"#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBracket,
                Token::True,
                Token::Comma,
                Token::False,
                Token::Comma,
                Token::Null,
                Token::RightBracket,
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let tokens = tokenize(r#"[42, -3.14, 0]"#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBracket,
                Token::NumberToken(42.0),
                Token::Comma,
                Token::NumberToken(-3.14),
                Token::Comma,
                Token::NumberToken(0.0),
                Token::RightBracket,
            ]
        );
    }

    #[test]
    fn test_unterminated_string() {
        let result = tokenize(r#""hello"#);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unterminated string")
        );
    }

    #[test]
    fn test_multiple_dots() {
        let result = tokenize(r#"[1.2.3]"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("multiple dots"));
    }

    #[test]
    fn test_unexpected_char() {
        let result = tokenize(r#"{@}"#);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unexpected character")
        );
    }
}
