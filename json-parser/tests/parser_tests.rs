use json_parser::tokenizer::tokenize;
use json_parser::parser::parse;

fn test_valid_file(filename: &str) {
    let content = std::fs::read_to_string(filename)
        .expect(&format!("Failed to read {}", filename));
    let tokens = tokenize(&content).expect(&format!("Failed to tokenize {}", filename));
    let result = parse(&tokens);
    assert!(result.is_ok(), "Expected {} to parse successfully, got: {:?}", filename, result.err());
}

fn test_invalid_file(filename: &str) {
    let content = std::fs::read_to_string(filename)
        .expect(&format!("Failed to read {}", filename));
    match tokenize(&content) {
        Ok(tokens) => {
            let result = parse(&tokens);
            assert!(result.is_err(), "Expected {} to fail parsing", filename);
        }
        Err(_) => {
            // tokenizer rejected invalid input, which is acceptable
        }
    }
}

#[test]
fn test_valid_simple_object() {
    test_valid_file("tests/valid_simple_object.json");
}

#[test]
fn test_valid_simple_array() {
    test_valid_file("tests/valid_simple_array.json");
}

#[test]
fn test_valid_empty_object() {
    test_valid_file("tests/valid_empty_object.json");
}

#[test]
fn test_valid_empty_array() {
    test_valid_file("tests/valid_empty_array.json");
}

#[test]
fn test_valid_primitives() {
    test_valid_file("tests/valid_primitives.json");
}

#[test]
fn test_valid_nested() {
    test_valid_file("tests/valid_nested.json");
}

#[test]
fn test_valid_array_of_objects() {
    test_valid_file("tests/valid_array_of_objects.json");
}

#[test]
fn test_valid_complex() {
    test_valid_file("tests/valid_complex.json");
}

#[test]
fn test_valid_string_with_escapes() {
    test_valid_file("tests/valid_string_with_escapes.json");
}

#[test]
fn test_invalid_missing_comma() {
    test_invalid_file("tests/invalid_missing_comma.json");
}

#[test]
fn test_invalid_missing_colon() {
    test_invalid_file("tests/invalid_missing_colon.json");
}

#[test]
fn test_invalid_missing_closing_brace() {
    test_invalid_file("tests/invalid_missing_closing_brace.json");
}

#[test]
fn test_invalid_missing_closing_bracket() {
    test_invalid_file("tests/invalid_missing_closing_bracket.json");
}

#[test]
fn test_invalid_trailing_comma_array() {
    test_invalid_file("tests/invalid_trailing_comma_array.json");
}

#[test]
fn test_invalid_trailing_comma_object() {
    test_invalid_file("tests/invalid_trailing_comma_object.json");
}

#[test]
fn test_invalid_unquoted_key() {
    test_invalid_file("tests/invalid_unquoted_key.json");
}

#[test]
fn test_invalid_single_quotes() {
    test_invalid_file("tests/invalid_single_quotes.json");
}

#[test]
fn test_invalid_missing_value() {
    test_invalid_file("tests/invalid_missing_value.json");
}
