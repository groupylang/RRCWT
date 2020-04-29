use super::Loc;
use super::token::TokenKind;
use super::lexer::LexError;
use super::parser::ParseError;
use super::error::Error;
use super::interpreter::InterpreterError;
use super::ir::{BasicBlock, Condition, InstrKind, Instruction};
use std::fmt;

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use super::token::TokenKind::*;
    match self {
      Number(n) => n.fmt(f),
      Plus => write!(f, "+"),
      Minus => write!(f, "-"),
      Asterisk => write!(f, "*"),
      Slash => write!(f, "/"),
      Less => write!(f, "<"),
      Equal => write!(f, "="),
      And => write!(f, "&"),
      Or => write!(f, "|"),
      Greater => write!(f, ">"),
      LParen => write!(f, "("),
      RParen => write!(f, ")"),
      LBrace => write!(f, "{{"),
      RBrace => write!(f, "}}"),
      LBracket => write!(f, "["),
      RBracket => write!(f, "]"),
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
    use super::lexer::LexErrorKind::*;
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
    use super::interpreter::InterpreterErrorKind::*;
    match self.value {
      DivisionByZero => write!(f, "division by zero"),
    }
  }
}

impl <'a> fmt::Debug for BasicBlock<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let result = writeln!(f, "{}:", self.label);
    for instr in &self.block {
      writeln!(f, "{:?}", instr).unwrap();
    }
    result
  }
}

impl <'a> fmt::Display for BasicBlock<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result = writeln!(f, "{}:", self.label);
    for instr in &self.block {
      writeln!(f, "{}", instr).unwrap();
    }
    result
  }
}

impl fmt::Debug for Condition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use Condition::*;
    match &self {
      GT { op1, op2 }=> write!(f, "gt r{}, r{}", op1, op2),
      GE { op1, op2 }=> write!(f, "ge r{}, r{}", op1, op2),
      EQ { op1, op2 }=> write!(f, "eq r{}, r{}", op1, op2),
      TRUE  => write!(f, "true"),
      FALSE => write!(f, "false"),
      // _ => { println!("error | InvalidCondKind"); Err(fmt::Error) }
    }
  }
}

impl <'a> fmt::Debug for Instruction<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use InstrKind::*;
    match &self.kind {
      RRR { op0, op1, op2 } => write!(f, "\tr{} = {} r{}, r{}", op0, self.mnemonic, op1, op2),
      RRI { op0, op1, op2 } => write!(f, "\tr{} = {} r{}, 0x{:02X}", op0, self.mnemonic, op1, op2),
      IF { cond, dst } => write!(f, "\t{} ({:?}) ${}", self.mnemonic, cond, dst),
      NEW { op0, size } => write!(f, "\tr{} = {} 0x{:04X}", op0, self.mnemonic, size),
      R { op0 } => write!(f, "\t{} r{}", self.mnemonic, op0),
      // _ => { println!("error | InvalidInstrKind"); Err(fmt::Error) }
    }
  }
}

impl <'a> fmt::Display for Instruction<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use InstrKind::*;
    match &self.kind {
      RRR { op0, op1, op2 } => write!(f, "{:02X}{:02X}{:02X}{:02X}", self.code, op0, op1, op2),
      RRI { op0, op1, op2 } => write!(f, "{:02X}{:02X}{:02X}{:02X}", self.code, op0, op1, op2),
      IF { cond, dst } => {
        use Condition::*;
        match cond {
          GT { op1, op2 } => write!(f, "44{}{:02X}{:02X}", dst, op1, op2),
          GE { op1, op2 } => write!(f, "45{}{:02X}{:02X}", dst, op1, op2),
          EQ { op1, op2 } => write!(f, "46{}{:02X}{:02X}", dst, op1, op2),
          TRUE  => write!(f, "40{}0000", dst),
          FALSE => write!(f, "00000000")
        }
      },
      NEW { op0, size } => write!(f, "{:02X}{:02X}{:04X}", self.code, op0, size),
      R { op0 }=> write!(f, "{:02X}{:02X}0000", self.code, op0),
      // _ => { println!("error | InvalidInstrKind"); Err(fmt::Error) }
    }
  }
}