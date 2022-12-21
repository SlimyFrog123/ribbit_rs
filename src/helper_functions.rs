use crate::token::{Token, TokenValue};

// Functions.
/// Print the given list of tokens.
pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens.iter() {
        match &token.value {
            TokenValue::Identifier(value) => {
                println!("Token: IDENTIFIER: {}", value);
            }
            TokenValue::Character(value) => {
                println!("Token: CHARACTER: {}", value);
            }
            TokenValue::String(value) => {
                println!("Token: STRING: {}", value);
            }
            TokenValue::Char(value) => {
                println!("Token: CHAR: {}", value);
            }
            TokenValue::Integer(value) => {
                println!("Token: INTEGER: {}", value);
            }
            TokenValue::Float(value) => {
                println!("Token: FLOAT: {}", value);
            }
            TokenValue::Boolean(value) => {
                println!("Token: BOOLEAN: {}", value);
            }
            TokenValue::EOF => {
                println!("Token: EOF");
            }
        }
    }
}