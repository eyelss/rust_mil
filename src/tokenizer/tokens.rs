use crate::shared::data_types::SupportedDataType;

#[derive(Debug)]
pub enum Sign {
  Add,
  Sub,
  Div,
  Mul,

  And,
  Or,

  Eq,
  Ne,
  Ge,
  Gt,
  Le,
  Lt,
  Not,
}

#[derive(Debug)]
pub enum Bracket {
  RoundOpen,
  RoundClose,
}

#[derive(Debug)]
pub enum Keyword {
  If,
  Else,
  EndIf,

  Each,
  As,
  EndEach,

  Skip,
}

#[derive(Debug)]
pub enum TokenError {
  Unknown(char),
  UnclosedStringSeq,
  UnclosedCodeSeq,
  WrongNumberSeq,
}

#[derive(Debug)]
pub enum Token {
  Raw(String),
  Literal(SupportedDataType),
  Identifier(String),
  
  Keyword(Keyword),
  Bracket(Bracket),
  Sign(Sign),
  // Unknown(char),
  Error(TokenError),
  Nothing,
  End
}