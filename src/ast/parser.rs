use crate::ast::lexer::{Lexer, Token, TokenKind};
use crate::ast::{ASTExpression, ASTExpressionKind, ASTStatement, ASTStatementKind};
use crate::diagnostics::DiagnosticsBagCell;

use super::{ASTBinaryOperator, ASTBinaryOperatorKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    diagnostics_bag: DiagnosticsBagCell,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics_bag: DiagnosticsBagCell) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|token| token.kind != TokenKind::Whitespace)
                .map(|token| token.clone())
                .collect(),
            current: 0,
            diagnostics_bag,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        return self.parse_statement();
    }

    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current()?;

        if token.kind == TokenKind::EOF {
            return None;
        }

        let expr: ASTExpression = self.parse_expression()?;
        Some(ASTStatement::new(ASTStatementKind::Expression(expr)))
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_binary_operator() {
            self.consume()?;
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence)?;
            left = ASTExpression::binary(operator, left, right);
        }

        Some(left)
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.peek(0)?;
        let kind = match token.kind {
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Plus),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Minus),
            TokenKind::Asterisk => Some(ASTBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Divide),
            _ => None,
        };

        kind.map(|kind| ASTBinaryOperator::new(kind, token.clone()))
    }

    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?;
        match token.kind {
            TokenKind::Number(number) => Some(ASTExpression::number(number)),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                let right_token = self.consume()?;
                if right_token.kind != TokenKind::RightParen {
                    return None;
                }
                Some(ASTExpression::parenthesized(expr))
            }
            _ => None,
        }
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        Some(token)
    }
}
