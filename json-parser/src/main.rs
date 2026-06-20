pub mod parser;
pub mod tokenizer;

use anyhow::Result;

fn main() -> Result<()> {
    let input = r#"{"name": "alice", "age": 30, "active": true, "x": [1, 2, 3], "y": {"a": "b"}}"#;
    let tokens = tokenizer::tokenize(input)?;
    let value = parser::parse(&tokens);

    println!("{:#?}", value);

    Ok(())
}
