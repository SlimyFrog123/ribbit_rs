// Declare internal modules.
mod lexer;
mod state;
mod token;
mod parser;
mod helper_functions;

// Internal modules.
use token::*;
use lexer::*;
use parser::*;
use helper_functions::*;

// Main function.
fn main() {
    let input: String = String::from("string myGreeting = \"Hello, world!\"; print(myGreeting);\
    int myInt = 3; print(myInt); myInt++; print(myInt); bool myBool = true; print(myBool);");

    let mut lexer: Lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.lex();

    print_tokens(&tokens);

    let mut parser: Parser = Parser::new(tokens);
    let ast: Vec<Node> = parser.parse();

}