use ast::lexer::Token;

use crate::ast::{lexer::Lexer, parser::Parser, Ast};

mod ast;

fn main() {
    let input = "7 + 8 * 9";
    // let input = "(42 * 5) + 10 / (3 - 1)";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
        tokens.push(token);
    }

    let mut ast = Ast::new();
    let mut parser = Parser::from_tokens(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();
}
