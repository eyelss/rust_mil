use crate::shared::data_types::SupportedDataType;
use crate::tokenizer::tokens::{
  Token,
  Bracket,
  TokenError,
  Keyword,
  Sign
};

#[derive(Debug)]
enum TokenizerState {
  Raw,
  Code
}

#[derive(Debug)]
pub struct Tokenizer {
  state: TokenizerState,
  raw_string: String,
  ptr: usize,
}

impl Tokenizer {
  fn rollback(&mut self) {
    self.ptr -= 1;
  }

  fn pass(&mut self) -> Option<char> {
    match self.raw_string.char_indices().nth(self.ptr) {
      Some((idx, ch)) => {
        self.ptr += 1;
        Some(ch)
      },
      _ => None
    }
  }

  fn skip_spaces(&mut self) {
    while let Some(ch) = self.pass() {
      if !ch.is_whitespace() {
        self.ptr -= 1;
        break;
      }
    }
  }

  fn tokenize_string(&mut self) -> Token {
    let ptr_start = self.ptr;

    while let Some(ch) = self.pass() {
      if ch == '"' {
        let str = &self.raw_string[ptr_start..self.ptr-1];

        return Token::Literal(SupportedDataType::String(String::from(str)));
      } 
    }

    return Token::Error(TokenError::UnclosedStringSeq);
  }

  fn tokenize_number(&mut self) -> Token {
    let ptr_start = self.ptr;
    let mut floated = false;

    while let Some(ch) = self.pass() {
      if ch == '.' {
        if floated {
          return Token::Error(TokenError::WrongNumberSeq);
        } else {
          floated = true;
        }
      } 

      if !ch.is_numeric() && ch != '.' {
        self.rollback();

        let str = &self.raw_string[ptr_start..self.ptr];
        
        if floated {
          let num_result = str.parse::<f64>();

          return match num_result {
            Ok(num) => Token::Literal(SupportedDataType::Float(num)),
            Err(_) => Token::Error(TokenError::WrongNumberSeq)
          }
        } else {
          
          let num_result = str.parse::<i64>();
          return match num_result {
            Ok(num) => Token::Literal(SupportedDataType::Integer(num)),
            Err(_) => Token::Error(TokenError::WrongNumberSeq)
          }
        }
      }
    }

    return Token::Error(TokenError::WrongNumberSeq);
  }

  fn tokenize_word(&mut self) -> Token {
    let ptr_start = self.ptr;

    while let Some(ch) = self.pass() {
      if !ch.is_alphabetic() && !ch.is_numeric() && ch != '_' && ch != '.' {
        self.rollback();
        break;
      }
    }

    let word = &self.raw_string[ptr_start..self.ptr];

    match word {
      "if" => Token::Keyword(Keyword::If),
      "else" => Token::Keyword(Keyword::Else),
      "endif" => Token::Keyword(Keyword::EndIf),
      "and" => Token::Keyword(Keyword::And),
      "or" => Token::Keyword(Keyword::Or),
      "each" => Token::Keyword(Keyword::Each),
      "as" => Token::Keyword(Keyword::As),
      "endeach" => Token::Keyword(Keyword::EndEach),
      "skip" => Token::Keyword(Keyword::Skip),
      "true" => Token::Literal(SupportedDataType::Boolean(true)),
      "false" => Token::Literal(SupportedDataType::Boolean(false)),
      another => Token::Identifier(String::from(another))
    }
  }

  fn tokenize_raw(&mut self) -> Token {
    let ptr_start = self.ptr;
    
    while let Some(ch) = self.pass() {
      if ch == '{' {
        match self.pass() {
            Some('{') => {
              self.state = TokenizerState::Code;
              if ptr_start == self.ptr - 2 {
                return Token::Nothing;
              }

              let str = &self.raw_string[ptr_start..self.ptr-2];
              return Token::Raw(String::from(str));
            },
            None => {
              if ptr_start == self.ptr {
                if self.ptr == self.raw_string.len() - 1 {
                  return Token::End;
                } else {
                  return Token::Raw(String::from(""));
                }
              } else {
                let str = &self.raw_string[ptr_start..self.ptr];
                return Token::Raw(String::from(str));
              }
            }
            _ => {} // pass
        } 
      }
    } 
    if ptr_start == self.ptr {
      return Token::End;
    } else {
      let str = &self.raw_string[ptr_start..self.ptr];
      return Token::Raw(String::from(str));
    }
  }

  fn tokenize_code(&mut self) -> Token {
    self.skip_spaces();

    match self.pass() {
      Some(c) => match c {
        '+' => Token::Sign(Sign::Add),
        '-' => Token::Sign(Sign::Sub),
        '/' => Token::Sign(Sign::Div),
        '*' => Token::Sign(Sign::Mul),
        '(' => Token::Bracket(Bracket::RoundOpen),
        ')' => Token::Bracket(Bracket::RoundClose),
        '<' => match self.pass() {
            Some('=') => Token::Sign(Sign::Le),
            None => Token::Sign(Sign::Lt),
            _ => {
              self.rollback();
              return Token::Sign(Sign::Lt);
            }
          },
        '>' => match self.pass() {
            Some('=') => Token::Sign(Sign::Ge),
            None => Token::Sign(Sign::Gt),
            _ => {
              self.rollback();
              return Token::Sign(Sign::Gt);
            }
          },
        '=' => match self.pass() {
            Some('=') => Token::Sign(Sign::Eq),
            None => Token::Error(TokenError::Unknown('=')),
            _ => {
              self.rollback();
              return Token::Error(TokenError::Unknown('='));
            }
          },
        '!' => match self.pass() {
            Some('=') => Token::Sign(Sign::Ne),
            None => Token::Sign(Sign::Not),
            _ => {
              self.rollback();
              return Token::Sign(Sign::Not);
            }
          },
        '}' => match self.pass() {
            Some('}') => {
              self.state = TokenizerState::Raw;
              return Token::Nothing;
            },
            Some(unknown) => Token::Error(TokenError::Unknown(unknown)),
            None => Token::End,
          },
        '"' => self.tokenize_string(),
        '0'..'9' => {
          self.rollback();
          self.tokenize_number()
        },
        x if x.is_alphabetic() || x == '_' => {
          self.rollback();
          self.tokenize_word()
        },
        unknown => Token::Error(TokenError::Unknown(unknown))
      },
      None => Token::End
    }
  }

  pub fn tokenize(&mut self) -> Token {
    match self.state {
      TokenizerState::Raw => self.tokenize_raw(),
      TokenizerState::Code => self.tokenize_code()
    }
  }

  pub fn new(raw_string: String) -> Self {
    Self {
      state: TokenizerState::Raw,
      raw_string: raw_string,
      ptr: 0usize,
    }
  }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
      let token = self.tokenize();
      match token {
        Token::Nothing => self.next(),
        Token::End => {
          match self.state {
            TokenizerState::Code => {
              self.state = TokenizerState::Raw;
              Some(Token::Error(TokenError::UnclosedCodeSeq))
            },
            _ => None
          }
        },
        token => Some(token)
      }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_tokenize_empty() {
    let tokenizer = Tokenizer::new(String::from(""));

    let tokens: Vec<Token> = tokenizer.collect();

    assert_eq!(tokens.len(), 0);
  }

  #[test]
  fn should_tokenize_raw() {
    let _raw = String::from("
      Just a simple raw. And it could be messsy:$#@123982mjsa_-sad1```\"sadasd\"
      But there is nothing to be afraid of. {} teasing....a}}}}}}}{ { {
    ");

    let template = _raw.clone();

    let tokenizer = Tokenizer::new(template);

    let tokens: Vec<Token> = tokenizer.collect();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(
      tokens.first().unwrap(), 
      Token::Raw(_raw)
    ));
  }

  #[test]
  fn should_tokenize_identifier() {
    let _identifier_name = String::from("variable1_name2");

    let tokenizer = Tokenizer::new(String::from("{{variable1_name2}}"));

    let tokens: Vec<Token> = tokenizer.collect();

    assert_eq!(tokens.len(), 1);
    assert!(matches!(
      tokens.first().unwrap(), 
      Token::Identifier(_identifier_name)
    ));

  }

  #[test]
  fn should_tokenize_string() {
    let _string_value = String::from("Simple man");
    let tokenizer = Tokenizer::new(String::from("{{\"Simple man\"}}"));

    let tokens: Vec<Token> = tokenizer.collect();

    assert_eq!(tokens.len(), 1);
    assert!(matches!(
      tokens.first().unwrap(), 
      Token::Literal(SupportedDataType::String(_string_value))
    ));
  }

  #[test]
  fn should_tokenize_float() {
    let tokenizer = Tokenizer::new(String::from("{{15.0}}"));

    let tokens: Vec<Token> = tokenizer.collect();

    assert_eq!(tokens.len(), 1);
    assert!(matches!(
      tokens.first().unwrap(), 
      Token::Literal(SupportedDataType::Float(15.0))
    ));
  }

  #[test]
  fn should_tokenize_integer() {
    let tokenizer = Tokenizer::new(String::from("{{150}}"));

    let tokens: Vec<Token> = tokenizer.collect();

    assert_eq!(tokens.len(), 1);
    assert!(matches!(
      tokens.first().unwrap(), 
      Token::Literal(SupportedDataType::Integer(150))
    ));
  }

  #[test]
  fn should_tokenize_boolean() {
    let tokenizer = Tokenizer::new(String::from("{{true}}"));

    let tokens: Vec<Token> = tokenizer.collect();
    
    assert_eq!(tokens.len(), 1);
    assert!(matches!(
      tokens.first().unwrap(), 
      Token::Literal(SupportedDataType::Boolean(true))
    ));
  }

  #[test]
  fn should_tokenize_raw_between() {
    let tokenizer = Tokenizer::new(String::from(" {{var_1}} {{var_2}} "));

    let tokens: Vec<Token> = tokenizer.collect();
    
    assert_eq!(tokens.len(), 5);

    let raw = String::from(" ");
    let var1_value = String::from("var_1");
    let var2_value = String::from("var_2");

    match &tokens[..] {
      [raw1, var1, raw2, var2, raw3] => {
        assert!(matches!(raw1, Token::Raw(raw)));
        assert!(matches!(raw2, Token::Raw(raw)));
        assert!(matches!(raw3, Token::Raw(raw)));
        assert!(matches!(var1, Token::Identifier(var1_value)));
        assert!(matches!(var2, Token::Identifier(var2_value)));
      },
      _ => {
        assert!(false);
      }
    }
  }

  #[test]
  fn should_tokenize_unclosed_string() {
    let tokenizer = Tokenizer::new(String::from("{{\"like of string}}"));
    
    let tokens: Vec<Token> = tokenizer.collect();
    
    assert!(matches!(
      tokens.first().unwrap(),
      Token::Error(TokenError::UnclosedStringSeq)
    ));
  }
}