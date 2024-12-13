use std::collections::HashMap;

use crate::ast::{
    lexer::TextSpan, ASTBinaryExpression, ASTBinaryOperatorKind, ASTExpression, ASTLetStatement,
    ASTNumberExpression, ASTParenthesizedExpression, ASTVariableExpression, ASTVisitor,
};

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
    pub variables: HashMap<String, i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self {
            last_value: None,
            variables: HashMap::new(),
        }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_variable_expression(&mut self, expr: &ASTVariableExpression) {
        let name = expr.token.span.literal.clone();
        let value = self.variables.get(&name).unwrap();
        self.last_value = Some(*value);
    }

    fn visit_parenthesized_expression(&mut self, expr: &ASTParenthesizedExpression) {
        self.visit_expression(&expr.expression);
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);
        let name = let_statement.identifier.span.literal.clone();
        let value = self.last_value.unwrap();
        self.variables.insert(name, value);
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&expr.right);
        let right = self.last_value.unwrap();

        match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => self.last_value = Some(left + right),
            ASTBinaryOperatorKind::Minus => self.last_value = Some(left - right),
            ASTBinaryOperatorKind::Multiply => self.last_value = Some(left * right),
            ASTBinaryOperatorKind::Divide => self.last_value = Some(left / right),
        }
    }

    fn visit_error(&mut self, span: &TextSpan) {
        todo!()
    }
}
