mod parser;
mod tokenizer;

use anyhow::Result;

fn main() -> Result<()> {
    let input = r#"{"name": "alice", "age": 30, "active": true}"#;
    let tokens = tokenizer::tokenize(input)?;
    let value = parser::parse(&tokens);

    Ok(())
}
