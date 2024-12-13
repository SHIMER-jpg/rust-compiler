#![allow(warnings)]
#![allow(non_exhaustive_patterns)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use diagnostics::{printer::DiagnosticsPrinter, DiagnosticBag, DiagnosticsBagCell};
use text::SourceText;

use crate::ast::{
    evaluator::ASTEvaluator,
    lexer::{Lexer, Token, TokenKind},
    parser::Parser,
    ASTLetStatement, ASTVariableExpression, ASTVisitor, Ast,
};

mod ast;
mod diagnostics;
mod text;

struct SymbolChecker {
    symbols: HashMap<String, ()>,
    diagnostics_bag: DiagnosticsBagCell,
}

impl SymbolChecker {
    pub fn new(diagnostics_bag: DiagnosticsBagCell) -> Self {
        Self {
            symbols: HashMap::new(),
            diagnostics_bag,
        }
    }
}

impl ASTVisitor for SymbolChecker {
    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);
        self.symbols
            .insert(let_statement.identifier.span.literal.clone(), ());
    }

    fn visit_variable_expression(&mut self, variable: &ASTVariableExpression) {
        if self.symbols.get(&variable.token.span.literal).is_none() {
            self.diagnostics_bag
                .borrow_mut()
                .report_undeclared_variable(&variable.identifier());
        }
    }

    fn visit_number_expression(&mut self, number: &ast::ASTNumberExpression) {}

    fn visit_parenthesized_expression(&mut self, parenthesized: &ast::ASTParenthesizedExpression) {}

    fn visit_error(&mut self, span: &ast::lexer::TextSpan) {}
}

fn main() -> Result<(), ()> {
    // let input = "a + 10";
    let input = "
        let a = 10
        let b = 20
        let d = 10
        let d = (a + b) * d
    ";

    let text = SourceText::new(input.to_string());

    //LEXER
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        if token.kind != TokenKind::EOF && token.kind != TokenKind::Whitespace {
            // println!("{:?}", token);
        }
        tokens.push(token)
    }
    println!("");

    //PARSER
    let diagnostics_bag: DiagnosticsBagCell = Rc::new(RefCell::new(DiagnosticBag::new()));
    let mut ast = Ast::new();
    let mut parser = Parser::new(tokens, Rc::clone(&diagnostics_bag));

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();

    //SYMBOL CHECKER
    let mut symbol_checker = SymbolChecker::new(Rc::clone(&diagnostics_bag));
    ast.visit(&mut symbol_checker);
    check_diagnostics(&text, &diagnostics_bag)?;

    //EVALUATOR
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    check_diagnostics(&text, &diagnostics_bag)?;

    //PRINTER
    println!("{:?}", eval.last_value);
    Ok(())
}

fn check_diagnostics(text: &SourceText, diagnostics_bag: &DiagnosticsBagCell) -> Result<(), ()> {
    let diagnostics_binding = diagnostics_bag.borrow();
    if diagnostics_binding.diagnostics.len() > 0 {
        let diagnostics_printer = DiagnosticsPrinter::new(&text, &diagnostics_binding.diagnostics);
        diagnostics_printer.print();
        return Err(());
    }
    Ok(())
}
