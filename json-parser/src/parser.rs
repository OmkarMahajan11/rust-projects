use std::collections::HashMap;

use crate::tokenizer::Token;

pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

pub fn parse(tokens: &[Token]) -> Json {
    todo!()
}
