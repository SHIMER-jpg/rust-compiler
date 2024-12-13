use lexer::{TextSpan, Token};
use termion::color::{self, Fg, Reset};

pub mod evaluator;
pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut impl ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = ASTPrinter::new();
        self.visit(&mut printer);
        println!("{}", printer.result);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
            ASTStatementKind::LetStatement(let_statement) => {
                self.visit_let_statement(let_statement)
            }
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement);

    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => self.visit_number_expression(number),
            ASTExpressionKind::Binary(binary) => self.visit_binary_expression(binary),
            ASTExpressionKind::Parenthesized(parenthesized) => {
                self.visit_parenthesized_expression(parenthesized)
            }
            ASTExpressionKind::Error(span) => self.visit_error(span),
            ASTExpressionKind::Variable(expr) => self.visit_variable_expression(expr),
        }
    }

    fn visit_variable_expression(&mut self, variable: &ASTVariableExpression);

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression);

    fn visit_binary_expression(&mut self, binary: &ASTBinaryExpression) {
        self.visit_expression(&binary.left);
        self.visit_expression(&binary.right);
    }

    fn visit_parenthesized_expression(&mut self, parenthesized: &ASTParenthesizedExpression) {
        self.visit_expression(&parenthesized.expression);
    }

    fn visit_error(&mut self, span: &TextSpan);
}

pub struct ASTPrinter {
    indent: usize,
    result: String,
}

const LEVEL_INDENT: usize = 2;

impl ASTPrinter {
    const NUMBER_COLOR: color::Yellow = color::Yellow;
    const TEXT_COLOR: color::LightWhite = color::LightWhite;
    const KEYWORD_COLOR: color::Blue = color::Blue;
    const VARIABLE_COLOR: color::Green = color::Green;
    fn add_whitespace(&mut self) {
        self.result.push_str(" ")
    }

    fn add_newline(&mut self) {
        self.result.push_str("\n")
    }

    pub fn new() -> Self {
        Self {
            indent: 0,
            result: String::new(),
        }
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_variable_expression(&mut self, expr: &ASTVariableExpression) {
        self.result.push_str(&format!(
            "{}{}",
            Self::VARIABLE_COLOR.fg_str(),
            expr.identifier().span.literal
        ));
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.result
            .push_str(&format!("{}let", Self::KEYWORD_COLOR.fg_str()));
        self.add_whitespace();

        self.result.push_str(&format!(
            "{}{}",
            Self::TEXT_COLOR.fg_str(),
            let_statement.identifier.span.literal
        ));

        self.add_whitespace();
        self.result
            .push_str(&format!("{}=", Self::TEXT_COLOR.fg_str()));
        self.add_whitespace();
        self.visit_expression(&let_statement.initializer);
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
        self.result.push_str(&format!("{}\n", Fg(Reset)));
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
        self.result
            .push_str(&format!("{}{}", Self::NUMBER_COLOR.fg_str(), number.number));
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), span.literal));
    }

    fn visit_binary_expression(&mut self, binary: &ASTBinaryExpression) {
        self.visit_expression(&binary.left);
        self.add_whitespace();

        self.result.push_str(&format!(
            "{}{}",
            Self::TEXT_COLOR.fg_str(),
            binary.operator.token.span.literal
        ));
        self.add_whitespace();
        self.visit_expression(&binary.right);
    }

    fn visit_parenthesized_expression(&mut self, parenthesized: &ASTParenthesizedExpression) {
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), "("));
        self.visit_expression(&parenthesized.expression);
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), ")"));
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self, s: &str) {
        println!("{}{}", " ".repeat(self.indent), s);
    }
}

//Statement
pub enum ASTStatementKind {
    Expression(ASTExpression),
    LetStatement(ASTLetStatement),
}

pub struct ASTLetStatement {
    pub identifier: Token,
    pub initializer: ASTExpression,
}

pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }

    pub fn let_statement(identifier: Token, initializer: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::LetStatement(ASTLetStatement {
            identifier,
            initializer,
        }))
    }
}

//Expression
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Variable(ASTVariableExpression),
    Error(TextSpan),
}

pub struct ASTVariableExpression {
    pub token: Token,
}

impl ASTVariableExpression {
    pub fn identifier(&self) -> &Token {
        &self.token
    }
}

pub struct ASTNumberExpression {
    number: i64,
}

pub struct ASTParenthesizedExpression {
    expression: Box<ASTExpression>,
}

#[derive(Debug)]
pub enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: lexer::Token,
}

impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: lexer::Token) -> Self {
        ASTBinaryOperator { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Plus | ASTBinaryOperatorKind::Minus => 1,
            ASTBinaryOperatorKind::Multiply | ASTBinaryOperatorKind::Divide => 2,
        }
    }
}

pub struct ASTBinaryExpression {
    operator: ASTBinaryOperator,
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }

    pub fn error(span: TextSpan) -> Self {
        ASTExpression::new(ASTExpressionKind::Error(span))
    }

    pub fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }))
    }

    pub fn identifier(token: Token) -> Self {
        ASTExpression::new(ASTExpressionKind::Variable(ASTVariableExpression { token }))
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }
}
