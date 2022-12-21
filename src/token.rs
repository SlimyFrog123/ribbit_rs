// Token Value (enum).
pub enum TokenValue {
    Identifier(String),
    Character(char),

    String(String),
    Char(char),
    Integer(i32),
    Float(f32),
    Boolean(bool),

    EOF
}

// Token (struct & implementation).
/// The tokens is a basic component of source code.
pub struct Token {
    pub(crate) value: TokenValue,
    position: i32
}

impl Token {
    pub fn new(value: TokenValue, position: i32) -> Token {
        return Token {
            value,
            position
        };
    }
}
