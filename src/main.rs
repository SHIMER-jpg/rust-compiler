#![allow(warnings)]
#![allow(non_exhaustive_patterns)]

use std::{cell::RefCell, rc::Rc};

use diagnostics::{DiagnosticBag, DiagnosticsBagCell};

use crate::ast::{
    evaluator::ASTEvaluator,
    lexer::{Lexer, Token, TokenKind},
    parser::Parser,
    Ast,
};

mod ast;
mod diagnostics;

fn main() {
    let input = "(7 - 2) * (30 + 7) * 8 & 2 ";
    // let input = "(42 * 5) + 10 / (3 - 1)";

    //LEXER
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        if token.kind != TokenKind::EOF && token.kind != TokenKind::Whitespace {
            println!("{:?}", token);
        }
        tokens.push(token)
    }

    let diagnostics_bag: DiagnosticsBagCell = Rc::new(RefCell::new(DiagnosticBag::new()));
    //PARSER
    let mut ast = Ast::new();
    let mut parser = Parser::new(tokens, diagnostics_bag);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();

    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);

    println!("{:?}", eval.last_value);
}
