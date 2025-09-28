use crate::lexer::tokens::{Bracket, Compare, Keyword, SingleChar, Token, TokenError};
use crate::shared::data_types::PrimitiveType;

#[derive(Debug)]
enum TokenizerState {
    Raw,
    Code,
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
        match self.raw_string.chars().nth(self.ptr) {
            Some(ch) => {
                self.ptr += 1;
                Some(ch)
            }
            _ => None,
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.raw_string.chars().nth(self.ptr + offset)
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
                let str = &self.raw_string[ptr_start..self.ptr - 1];

                return Token::Literal(PrimitiveType::String(String::from(str)));
            }
        }

        Token::Error(TokenError::UnclosedStringSeq)
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

              return if floated {
                let num_result = str.parse::<f64>();

                match num_result {
                  Ok(num) => Token::Literal(PrimitiveType::Float(num)),
                  Err(_) => Token::Error(TokenError::WrongNumberSeq),
                }
              } else {
                let num_result = str.parse::<i64>();
                match num_result {
                  Ok(num) => Token::Literal(PrimitiveType::Integer(num)),
                  Err(_) => Token::Error(TokenError::WrongNumberSeq),
                }
              }
            }
        }

        Token::Error(TokenError::WrongNumberSeq)
    }

    fn tokenize_word(&mut self) -> Token {
        let ptr_start = self.ptr;

        while let Some(ch) = self.pass() {
            if !ch.is_alphabetic() && !ch.is_numeric() && ch != '_' {
                self.rollback();
                break;
            }
        }

        let word = &self.raw_string[ptr_start..self.ptr];

        match word {
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "endif" => Token::Keyword(Keyword::EndIf),
            "each" => Token::Keyword(Keyword::Each),
            "as" => Token::Keyword(Keyword::As),
            "endeach" => Token::Keyword(Keyword::EndEach),
            "skip" => Token::Keyword(Keyword::Skip),
            "true" => Token::Literal(PrimitiveType::Boolean(true)),
            "false" => Token::Literal(PrimitiveType::Boolean(false)),
            another => Token::Identifier(String::from(another)),
        }
    }

    fn tokenize_raw(&mut self) -> Token {
        let ptr_start = self.ptr;

        while let Some(ch) = self.pass() {
            if ch == '{' {
                match self.peek(0) {
                    Some('{') => {
                        self.state = TokenizerState::Code;
                        if ptr_start == self.ptr - 1 {
                            return Token::Nothing;
                        }

                        let _ = &self.rollback();
                        let str = &self.raw_string[ptr_start..self.ptr];
                        return Token::Raw(String::from(str));
                    }
                    None => {
                      return if ptr_start == self.ptr {
                        if self.ptr == self.raw_string.len() - 1 {
                          Token::End
                        } else {
                          Token::Raw(String::from(""))
                        }
                      } else {
                        let str = &self.raw_string[ptr_start..self.ptr];
                        Token::Raw(String::from(str))
                      }
                    }
                    _ => {}
                }
            }
        }
        if ptr_start == self.ptr {
            Token::End
        } else {
            let str = &self.raw_string[ptr_start..self.ptr];
            Token::Raw(String::from(str))
        }
    }

    fn tokenize_code(&mut self) -> Token {
        self.skip_spaces();

        match self.pass() {
            Some(c) => match c {
                '+' => Token::Single(SingleChar::Plus),
                '-' => Token::Single(SingleChar::Minus),
                '/' => Token::Single(SingleChar::Slash),
                '*' => Token::Single(SingleChar::Star),
                '&' => Token::Single(SingleChar::Ampersand),
                '|' => Token::Single(SingleChar::Pipe),
                '.' => Token::Single(SingleChar::Dot),
                '(' => Token::Bracket(Bracket::RoundOpen),
                ')' => Token::Bracket(Bracket::RoundClose),
                '<' => match self.pass() {
                    Some('=') => Token::Compare(Compare::Le),
                    None => Token::Compare(Compare::Lt),
                    _ => {
                        self.rollback();
                        Token::Compare(Compare::Lt)
                    }
                },
                '>' => match self.pass() {
                    Some('=') => Token::Compare(Compare::Ge),
                    None => Token::Compare(Compare::Gt),
                    _ => {
                        self.rollback();
                        Token::Compare(Compare::Gt)
                    }
                },
                '=' => match self.pass() {
                    Some('=') => Token::Compare(Compare::Eq),
                    None => Token::Error(TokenError::Unknown('=')),
                    _ => {
                        self.rollback();
                        Token::Error(TokenError::Unknown('='))
                    }
                },
                '!' => match self.pass() {
                    Some('=') => Token::Compare(Compare::Ne),
                    None => Token::Single(SingleChar::Excl),
                    _ => {
                        self.rollback();
                        Token::Single(SingleChar::Excl)
                    }
                },
                '}' => match self.pass() {
                    Some('}') => {
                        self.state = TokenizerState::Raw;
                        Token::CloseTag
                    }
                    Some(unknown) => Token::Error(TokenError::Unknown(unknown)),
                    None => Token::End,
                },
                '{' => match self.pass() {
                    Some('{') => Token::OpenTag,
                    Some(unknown) => Token::Error(TokenError::Unknown(unknown)),
                    None => Token::End,
                },
                '"' => self.tokenize_string(),
                '0'..'9' => {
                    self.rollback();
                    self.tokenize_number()
                }
                x if x.is_alphabetic() || x == '_' => {
                    self.rollback();
                    self.tokenize_word()
                }
                unknown => Token::Error(TokenError::Unknown(unknown)),
            },
            None => Token::End,
        }
    }

    pub fn tokenize(&mut self) -> Token {
        match self.state {
            TokenizerState::Raw => self.tokenize_raw(),
            TokenizerState::Code => self.tokenize_code(),
        }
    }

    pub fn new(raw_string: String) -> Self {
        Self {
            state: TokenizerState::Raw,
            raw_string,
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
            Token::End => match self.state {
                TokenizerState::Code => {
                    self.state = TokenizerState::Raw;
                    Some(Token::Error(TokenError::UnclosedCodeSeq))
                }
                _ => None,
            },
            token => Some(token),
        }
    }
}
