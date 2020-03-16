use super::{Loc, TokenKind};
use super::lexer::{LexErrorKind, LexError};
use super::parser::ParseError;
use super::error::Error;
use super::interpreter::{InterpreterErrorKind, InterpreterError};
use std::fmt;

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::TokenKind::*;
    match self {
      Number(n) => n.fmt(f),
      Plus => write!(f, "+"),
      Minus => write!(f, "-"),
      Asterisk => write!(f, "*"),
      Slash => write!(f, "/"),
      LParen => write!(f, "("),
      RParen => write!(f, ")"),
    }
  }
}

impl fmt::Display for Loc {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}-{}", self.0, self.1)
  }
}

impl fmt::Display for LexError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::LexErrorKind::*;
    let loc = &self.loc;
    match self.value {
      InvalidChar(c) => write!(f, "{}: invalid char '{}'", loc, c),
      Eof => write!(f, "End of file"),
    }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::ParseError::*;
    match self {
      UnexpectedToken(tok) => write!(f, "{}: {} is not expected", tok.loc, tok.value),
      NotExpression(tok) => write!(
        f,
        "{}: '{}' is not a start of expression",
        tok.loc, tok.value
      ),
      NotOperator(tok) => write!(f, "{}: '{}' is not an operator", tok.loc, tok.value),
      UnclosedOpenParen(tok) => write!(f, "{}: '{}' is not closed", tok.loc, tok.value),
      RedundantExpression(tok) => write!(
        f,
        "{}: expression after '{}' is redundant",
        tok.loc, tok.value
      ),
      Eof => write!(f, "End of file"),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "parser error")
  }
}

impl fmt::Display for InterpreterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::InterpreterErrorKind::*;
    match self.value {
      DivisionByZero => write!(f, "division by zero"),
    }
  }
}