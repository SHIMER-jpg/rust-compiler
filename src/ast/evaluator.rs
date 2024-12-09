use crate::ast::{
    ASTBinaryExpression, ASTBinaryOperatorKind, ASTExpression, ASTNumberExpression, ASTVisitor,
};

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary(&mut self, expr: &ASTBinaryExpression) {
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
}
