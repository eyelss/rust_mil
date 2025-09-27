use crate::shared::data_types::PrimitiveType;
use super::expression::{Expression, CalculatableExpression};

pub trait Visitable {
  fn accept(&self, visitor: &Visitor) -> String;
}

pub struct Visitor;

impl Visitor {
  pub fn parenthesize(&self, calc: &CalculatableExpression) -> String {
    match calc {
      CalculatableExpression::Binary(binary_expression) => {
        String::from(format!(
          "({:?} {} {})",
          binary_expression.operation,
          binary_expression.left.accept(self),
          binary_expression.right.accept(self)
        ))
      },
      CalculatableExpression::Unary(unary_expression) => {
        String::from(format!(
          "({:?} {})",
          unary_expression.operation,
          unary_expression.argument.accept(self)
        ))
      },
  }
  }

  pub fn visit(&self, e: &Expression) -> String {
    match e {
      Expression::Inner(calculatable) => 
        self.parenthesize(calculatable),
      Expression::Leaf(primitive) => {
        match primitive {
            PrimitiveType::String(v) => v.to_string(),
            PrimitiveType::Boolean(v) => v.to_string(),
            PrimitiveType::Integer(v) => v.to_string(),
            PrimitiveType::Float(v) => v.to_string(),
            PrimitiveType::Null => String::from("null"),
        }
      }
  }
  }
}