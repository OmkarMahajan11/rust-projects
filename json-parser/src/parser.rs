use crate::tokenizer::Token;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

pub fn parse(tokens: &[Token]) -> Result<Json> {
    let (j, consumed) = _parse(tokens, 0)?;
    if consumed != tokens.len() {
        anyhow::bail!("unexpected content after valid JSON value");
    }
    Ok(j)
}

fn _parse(tokens: &[Token], idx: usize) -> Result<(Json, usize)> {
    if idx < tokens.len() {
        match &tokens[idx] {
            Token::LeftBrace => parse_object(tokens, idx),
            Token::LeftBracket => parse_array(tokens, idx),
            Token::StringToken(s) => Ok((Json::String(String::from(s)), idx + 1)),
            Token::NumberToken(n) => Ok((Json::Number(*n), idx + 1)),
            Token::True => Ok((Json::Bool(true), idx + 1)),
            Token::False => Ok((Json::Bool(false), idx + 1)),
            Token::Null => Ok((Json::Null, idx + 1)),
            _ => anyhow::bail!("error"),
        }
    } else {
        anyhow::bail!("unexpected end of input")
    }
}

fn parse_object(tokens: &[Token], mut idx: usize) -> Result<(Json, usize)> {
    let mut m: HashMap<String, Json> = HashMap::new();

    // consume '{'
    idx += 1;

    // for every iteration of loop, one `key: val` is parsed
    let mut closed = false;
    while idx < tokens.len() {
        match &tokens[idx] {
            Token::RightBrace => {
                idx += 1;
                closed = true;
                break;
            }
            Token::Comma => {
                idx += 1;
                if idx >= tokens.len() {
                    anyhow::bail!("unexpected end of input after `,`");
                }
                if matches!(&tokens[idx], Token::RightBrace | Token::RightBracket) {
                    anyhow::bail!("trailing commas are not allowed");
                }
                continue;
            }
            Token::StringToken(s) => {
                let key = String::from(s);
                idx += 1;

                if idx >= tokens.len() {
                    anyhow::bail!("unexpected end of input, expected `:`");
                }
                if &tokens[idx] != &Token::Colon {
                    anyhow::bail!("expected a `:`");
                }

                // consume `:`
                idx += 1;

                let (val, i) = _parse(tokens, idx)?;
                m.insert(key, val);
                idx = i;
            }
            _ => anyhow::bail!("only strings can be object keys"),
        }
    }

    if !closed {
        anyhow::bail!("unexpected end of input, expected `}}`");
    }

    Ok((Json::Object(m), idx))
}

fn parse_array(tokens: &[Token], mut idx: usize) -> Result<(Json, usize)> {
    let mut arr = Vec::new();

    // consume '['
    idx += 1;

    let mut closed = false;
    while idx < tokens.len() {
        match &tokens[idx] {
            Token::RightBracket => {
                // consume ']'
                idx += 1;
                closed = true;
                break;
            }
            Token::Comma => {
                idx += 1;
                continue;
            }
            _ => match _parse(tokens, idx)? {
                (j, i) => {
                    idx = i;
                    arr.push(j);
                }
            },
        }
    }

    if !closed {
        anyhow::bail!("unexpected end of input, expected `]`");
    }

    Ok((Json::Array(arr), idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_object() {
        let tokens = vec![
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
        ];

        let result = parse(&tokens);
        assert!(
            result.is_ok(),
            "Expected valid object to parse successfully"
        );

        let json = result.unwrap();
        match json {
            Json::Object(map) => {
                assert_eq!(map.len(), 3);
                assert!(map.contains_key("name"));
                assert!(map.contains_key("age"));
                assert!(map.contains_key("active"));

                if let Some(Json::String(s)) = map.get("name") {
                    assert_eq!(s, "alice");
                } else {
                    panic!("Expected 'name' to be String(\"alice\")");
                }

                if let Some(Json::Number(n)) = map.get("age") {
                    assert_eq!(*n, 30.0);
                } else {
                    panic!("Expected 'age' to be Number(30.0)");
                }

                if let Some(Json::Bool(b)) = map.get("active") {
                    assert_eq!(*b, true);
                } else {
                    panic!("Expected 'active' to be Bool(true)");
                }
            }
            _ => panic!("Expected Json::Object"),
        }
    }

    #[test]
    fn test_parse_object_missing_right_brace() {
        let tokens = vec![
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
            // Missing Token::RightBrace
        ];

        let result = parse(&tokens);
        assert!(result.is_err(), "Expected error for missing closing brace");
    }

    #[test]
    fn test_parse_array() {
        let tokens = vec![
            Token::LeftBracket,
            Token::NumberToken(42.0),
            Token::Comma,
            Token::NumberToken(-3.14),
            Token::Comma,
            Token::NumberToken(0.0),
            Token::RightBracket,
        ];

        let result = parse(&tokens);
        assert!(result.is_ok(), "Expected valid array to parse successfully");

        if let Json::Array(arr) = result.unwrap() {
            assert_eq!(arr.len(), 3);

            if let Json::Number(n) = &arr[0] {
                assert_eq!(*n, 42.0);
            } else {
                panic!("Expected arr[0] to be Number(42.0)");
            }

            if let Json::Number(n) = &arr[1] {
                assert_eq!(*n, -3.14);
            } else {
                panic!("Expected arr[1] to be Number(-3.14)");
            }

            if let Json::Number(n) = &arr[2] {
                assert_eq!(*n, 0.0);
            } else {
                panic!("Expected arr[2] to be Number(0.0)");
            }
        } else {
            panic!("Expected Json::Array");
        }
    }
}
