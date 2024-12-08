use ast::lexer::Token;

mod ast;

fn main() {
    let input = "(42 * 5) + 10 / (3 - 1)";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
        tokens.push(token);
    }
}
