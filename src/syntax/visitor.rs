use super::expression::{CalculatableExpression, Expression, LeafedExpression};
use crate::shared::data_types::PrimitiveType;

pub trait Visitable {
    fn accept(&self, visitor: &Visitor) -> String;
}

pub struct Visitor;

impl Visitor {
    pub fn parenthesize(&self, calc: &CalculatableExpression) -> String {
        match calc {
            CalculatableExpression::Binary(binary_expression) => String::from(format!(
                "({:?} {} {})",
                "stub",
                binary_expression.left.accept(self),
                binary_expression.right.accept(self)
            )),
            CalculatableExpression::Unary(unary_expression) => String::from(format!(
                "({:?} {})",
                unary_expression.operation,
                unary_expression.argument.accept(self)
            )),
        }
    }

    pub fn visit(&self, e: &Expression) -> String {
        match e {
            Expression::Inner(calculatable) => self.parenthesize(calculatable),
            Expression::Leaf(leaf) => match leaf {
                LeafedExpression::Literal(primitive) => match primitive {
                    PrimitiveType::String(v) => v.to_string(),
                    PrimitiveType::Boolean(v) => v.to_string(),
                    PrimitiveType::Integer(v) => v.to_string(),
                    PrimitiveType::Float(v) => v.to_string(),
                    PrimitiveType::Null => String::from("null"),
                },
                LeafedExpression::Identifier(id) => id.join("."),
            },
        }
    }
}
