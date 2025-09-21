use crate::shared::data_types::SupportedDataType;

#[derive(Debug)]
pub enum SingleChar {
  Plus,
  Minus,
  Slash,
  Star,

  Ampersand,
  Pipe,
  Excl,
}

#[derive(Debug)]
pub enum Compare {
  Eq,
  Ne,
  Ge,
  Gt,
  Le,
  Lt,
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
  Single(SingleChar),
  Compare(Compare),
  // Sign(Sign),
  // Unknown(char),
  Error(TokenError),
  Nothing,
  End
}