use regex::Regex;

use crate::state::State;
use crate::token::{Token, TokenValue};

// Lexer.
/// The Lexer to turn the plain text given into a list of tokens that the program can actually
/// understand, and eventually pass to other parts of the language.
pub struct Lexer {
    input: String,
    state: State,
    position: i32,
    current_char: char
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        return Lexer {
            input,
            state: State::Go,
            position: -1,
            current_char: '\0'
        };
    }

    fn advance(&mut self) {
        self.position += 1;

        if self.position >= self.input.len() as i32 {
            self.state = State::Stop;
            self.current_char = '\0';
        }
        else {
            self.current_char = self.input.chars().nth(self.position as usize).unwrap();
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        // Regular Expressions.
        let number: Regex = Regex::new(r"[0-9]").unwrap();
        let number_full: Regex = Regex::new(r"[0-9.]").unwrap();
        let identifier: Regex = Regex::new(r"[a-zA-Z_]").unwrap();
        let identifier_full: Regex = Regex::new(r"[0-9a-zA-Z_]").unwrap();
        let discard: Regex = Regex::new(r"[ \r\n\t]").unwrap(); // Characters to discard.

        // Advance once, to get the first character.
        self.advance();

        // Generate the tokens.
        while self.state == State::Go {
            if number.is_match(&String::from(self.current_char)) {
                // The literal string of the number to later parse.
                let mut string_value: String = String::new();
                // The count of dots (decimal points) in the number.
                let mut dot_count: i32 = 0;

                while number_full.is_match(&String::from(self.current_char)) {
                    if self.current_char == '.' {
                        dot_count += 1;

                        if dot_count > 1 {
                            // Panic (stop the program) if there are more than one decimal
                            // points in a number, as well as throw an error message.
                            panic!("Cannot have more than one decimal point in a number!");
                        }
                    }

                    string_value.push(self.current_char);
                    self.advance();
                }

                let token: Token;

                if dot_count == 1 {
                    token = Token::new(
                        TokenValue::Float(string_value.parse::<f32>().unwrap()),
                        self.position
                    );
                }
                else if dot_count == 0 {
                    token = Token::new(
                        TokenValue::Integer(string_value.parse::<i32>().unwrap()),
                        self.position
                    );
                }
                else {
                    // If the number has an odd amount of decimal points, panic and throw an
                    // error.

                    panic!("An unexpected error occurred.");
                }

                tokens.push(token);

                continue;
            }
            else if identifier.is_match(&String::from(self.current_char)) {
                let mut identifier_name: String = String::new();

                while identifier_full.is_match(&String::from(self.current_char)) {
                    identifier_name.push(self.current_char);
                    self.advance();
                }

                if identifier_name == "true" || identifier_name == "false" {
                    let value: bool = match identifier_name.as_str() {
                        "true" => true,
                        "false" => false,
                        _ =>  {
                            panic!("An unexpected error occurred.");
                        }
                    };

                    let token: Token = Token::new(
                        TokenValue::Boolean(value),
                        self.position
                    );
                    tokens.push(token);

                    continue;
                }

                let token: Token = Token::new(
                    TokenValue::Identifier(identifier_name),
                    self.position
                );
                tokens.push(token);

                continue;
            }
            else if self.current_char == '"' {
                // String.

                // The contents of the string.
                let mut contents: String = String::new();
                let mut last_char: char = '\0';
                let mut closed: bool = false;

                self.advance();

                while self.state == State::Go {
                    if self.current_char == '"' {
                        if !(last_char == '\\') {
                            closed = true;
                            self.advance();
                            break;
                        }
                    }

                    contents.push(self.current_char);
                    last_char = self.current_char;
                    self.advance();
                }

                // Add the string token.
                let token: Token = Token::new(
                    TokenValue::String(contents),
                    self.position - 1
                );
                tokens.push(token);

                if !closed {
                    panic!("Unclosed string!");
                }

                continue;
            }
            // else if self.current_char == '\'' {
            //     // Single Character.
            //     let mut character: char = '\0';
            //
            //     self.advance();
            //
            //     if self.current_char == '\\' {
            //         self.advance();
            //
            //         character = self.current_char;
            //
            //         self.advance();
            //
            //         if self.current_char == '\'' {
            //             let mut token: Token = Token::new(
            //                 TokenValue::Char(character),
            //                 self.position
            //             );
            //             tokens.push(token);
            //
            //             self.advance();
            //
            //             continue
            //         }
            //
            //         panic!("An error occurred!");
            //     }
            //     else {
            //         character = self.current_char;
            //         self.advance();
            //
            //         if self.current_char == '\'' {
            //             let mut token: Token = Token::new(
            //                 TokenValue::Char(character),
            //                 self.position
            //             );
            //             tokens.push(token);
            //
            //             self.advance();
            //
            //             continue
            //         }
            //
            //         panic!("An error occurred!");
            //     }
            // }

            if discard.is_match(&String::from(self.current_char)) {
                self.advance();
                continue;
            }

            // Append the current character as a type of TokenValue::Character.
            let token: Token = Token::new(
                TokenValue::Character(
                    self.current_char
                ),
                self.position
            );
            tokens.push(token);

            self.advance()
        }

        // Append the EOF token to show the program where the end it.
        let token: Token = Token::new(
            TokenValue::EOF,
            self.position
        );
        tokens.push(token);

        return tokens;
    }
}
