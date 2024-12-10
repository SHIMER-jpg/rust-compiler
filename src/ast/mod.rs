use lexer::TextSpan;

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
        let mut printer = ASTPrinter { indent: 0 };
        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => self.visit_number(number),
            ASTExpressionKind::Binary(binary) => self.visit_binary(binary),
            ASTExpressionKind::Parenthesized(parenthesized) => {
                self.visit_parenthesized(parenthesized)
            }
            ASTExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_number(&mut self, number: &ASTNumberExpression);

    fn visit_binary(&mut self, binary: &ASTBinaryExpression) {
        self.visit_expression(&binary.left);
        self.visit_expression(&binary.right);
    }

    fn visit_parenthesized(&mut self, parenthesized: &ASTParenthesizedExpression) {
        self.visit_expression(&parenthesized.expression);
    }

    fn visit_error(&mut self, span: &TextSpan);
}

pub struct ASTPrinter {
    indent: usize,
}

const LEVEL_INDENT: usize = 2;

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("Statement");
        self.indent += LEVEL_INDENT;
        self.do_visit_statement(statement);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("Expression");
        self.indent += LEVEL_INDENT;
        self.do_visit_expression(expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number: {}", number.number));
    }

    fn visit_binary(&mut self, binary: &ASTBinaryExpression) {
        self.print_with_indent("Binary Expression: ");
        self.indent += LEVEL_INDENT;
        self.print_with_indent(&format!("Operator: {:?}", binary.operator.kind));
        self.visit_expression(&binary.left);
        self.visit_expression(&binary.right);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_parenthesized(&mut self, parenthesized: &ASTParenthesizedExpression) {
        self.print_with_indent("Parenthesized Expression: ");
        self.indent += LEVEL_INDENT;
        self.visit_expression(&parenthesized.expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.print_with_indent(&format!("Error: {:?}", span));
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
}

//Expression
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Error(TextSpan),
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

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }
}
