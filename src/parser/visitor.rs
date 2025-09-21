use crate::{parser::lexer::*, shared::data_types::SupportedDataType};

pub struct ASTVisitor {

}

// impl ASTVisitor {
//     pub fn visit(&mut self, node: ASTNode) -> u32 {
//       return match node {
//         ASTNode::Prgm(p) => {
//           let result = p.statements.iter().map(|stmt: &Statement| {
//             self.visit(ASTNode::Stmt(stmt))
//           });
//         },
//         ASTNode::Blck(b) => {
          
//         },
//         ASTNode::Stmt(s) => {
          
//         },
//         ASTNode::Expr(e) => {
//           let r = match e {
//             Expression::Inner(calc) => {
//               match calc {
//                 CalculatableExpression::Binary(b) => {
                  
//                 },
//                 CalculatableExpression::Unary(u) => {
//                   u.operation
//                 }
//               }
//             },
//             Expression::Leaf(term) => term,
//           };
//         }
//       }
//     }
// }