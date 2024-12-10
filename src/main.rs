#![allow(warnings)]
#![allow(non_exhaustive_patterns)]

use std::{cell::RefCell, rc::Rc};

use diagnostics::{printer::DiagnosticsPrinter, DiagnosticBag, DiagnosticsBagCell};
use text::SourceText;

use crate::ast::{
    evaluator::ASTEvaluator,
    lexer::{Lexer, Token, TokenKind},
    parser::Parser,
    Ast,
};

mod ast;
mod diagnostics;
mod text;

fn main() {
    let input = "(7 - 2) * (30 + 7) * 8 A 2 ";
    let text = SourceText::new(input.to_string());
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
    let mut parser = Parser::new(tokens, Rc::clone(&diagnostics_bag));

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();
    let diagnostics_binding = diagnostics_bag.borrow();
    if diagnostics_binding.diagnostics.len() > 0 {
        let diagnostics_printer = DiagnosticsPrinter::new(&text, &diagnostics_binding.diagnostics);
        diagnostics_printer.print();
        return;
    }

    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);

    println!("{:?}", eval.last_value);
}
