#![allow(warnings)]
#![allow(non_exhaustive_patterns)]

use ast::evaluator::ASTEvaluator;

use crate::ast::{
    lexer::{Lexer, Token, TokenKind},
    parser::Parser,
    Ast,
};

mod ast;

fn main() {
    let input = "(7 + 8) * 3";
    // let input = "(42 * 5) + 10 / (3 - 1)";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        if token.kind != TokenKind::EOF && token.kind != TokenKind::Whitespace {
            println!("{:?}", token);
        }
        tokens.push(token)
    }

    let mut ast = Ast::new();
    let mut parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();

    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);

    println!("{:?}", eval.last_value);
}
