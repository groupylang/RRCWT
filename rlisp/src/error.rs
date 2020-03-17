use super::{Loc, Token, Ast};
use super::lexer::{LexError, lex};
use super::parser::{ParseError, parse};
use std::str::FromStr;
use std::error::Error as StdError;
use super::interpreter::InterpreterError;


impl FromStr for Ast {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = lex(s)?;
    let ast = parse(tokens)?;
    Ok(ast)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
  Lexer(LexError),
  Parser(ParseError),
}

impl From<LexError> for Error {
  fn from(e: LexError) -> Self {
    Error::Lexer(e)
  }
}

impl From<ParseError> for Error {
  fn from(e: ParseError) -> Self {
    Error::Parser(e)
  }
}

impl StdError for LexError {}

impl StdError for ParseError {}

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    use self::Error::*;
    match self {
      Lexer(lex) => Some(lex),
      Parser(parse) => Some(parse),
    }
  }
}

pub fn print_annot(input: &str, loc: Loc) {
  eprintln!("{}", input);
  eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

impl Error {
  pub fn show_diagnostic(&self, input: &str) {
    use self::Error::*;
    use self::ParseError as P;
    let (e, loc): (&dyn StdError, Loc) = match self {
      Lexer(e) => (e, e.loc.clone()),
      Parser(e) => {
        let loc = match e {
          P::UnexpectedToken(Token { loc, .. })
          | P::NotExpression(Token { loc, .. })
          | P::NotOperator(Token { loc, .. })
          | P::UnclosedOpenParen(Token { loc, .. }) => loc.clone(),
          P::RedundantExpression(Token { loc, .. }) => Loc(loc.0, input.len()),
          P::Eof => Loc(input.len(), input.len() + 1),
        };
        (e, loc)
      }
    };
    eprintln!("{}", e);
    print_annot(input, loc);
  }
}

pub fn show_trace<E: StdError>(e: E) {
  eprintln!("{}", e);
  let mut source = e.source();
  while let Some(e) = source {
    eprintln!("caused by {}", e);
    source = e.source()
  }
}

impl StdError for InterpreterError {
  fn description(&self) -> &str {
    use super::interpreter::InterpreterErrorKind::*;
    match self.value {
      DivisionByZero => "the right hand expression of the division evaluates to zero",
    }
  }
}