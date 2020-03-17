pub mod lexer;
pub mod parser;
pub mod error;
pub mod disp;
pub mod interpreter;
pub mod compiler;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc(usize, usize);

impl Loc {
  fn merge(&self, other: &Loc) -> Loc {
    use std::cmp::{max, min};
    Loc(min(self.0, other.0), max(self.1, other.1))
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
  value: T,
  loc: Loc,
}

impl<T> Annot<T> {
  fn new(value: T, loc: Loc) -> Self {
    Self { value, loc }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
  Number(u64),
  Plus,
  Minus,
  Asterisk,
  Slash,
  LParen,
  RParen,
}

pub type Token = Annot<TokenKind>;

impl Token {
  fn number(n: u64, loc: Loc) -> Self {
    Self::new(TokenKind::Number(n), loc)
  }
  fn plus(loc: Loc) -> Self {
    Self::new(TokenKind::Plus, loc)
  }
  fn minus(loc: Loc) -> Self {
    Self::new(TokenKind::Minus, loc)
  }
  fn asterisk(loc: Loc) -> Self {
    Self::new(TokenKind::Asterisk, loc)
  }
  fn slash(loc: Loc) -> Self {
    Self::new(TokenKind::Slash, loc)
  }
  fn lparen(loc: Loc) -> Self {
    Self::new(TokenKind::LParen, loc)
  }
  fn rparen(loc: Loc) -> Self {
    Self::new(TokenKind::RParen, loc)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniOpKind {
  Plus,
  Minus,
}

pub type UniOp = Annot<UniOpKind>;

impl UniOp {
  fn plus(loc: Loc) -> Self {
    Self::new(UniOpKind::Plus, loc)
  }

  fn minus(loc: Loc) -> Self {
    Self::new(UniOpKind::Minus, loc)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOpKind {
  Add,
  Sub,
  Mult,
  Div,
}

pub type BinOp = Annot<BinOpKind>;

impl BinOp {
  fn add(loc: Loc) -> Self {
    Self::new(BinOpKind::Add, loc)
  }
  fn sub(loc: Loc) -> Self {
    Self::new(BinOpKind::Sub, loc)
  }
  fn mult(loc: Loc) -> Self {
    Self::new(BinOpKind::Mult, loc)
  }
  fn div(loc: Loc) -> Self {
    Self::new(BinOpKind::Div, loc)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
  Num(u64),
  UniOp { op: UniOp, e: Box<Ast> },
  BinOp { op: BinOp, l: Box<Ast>, r: Box<Ast> },
}

pub type Ast = Annot<AstKind>;

impl Ast {
  #[allow(dead_code)]
  fn num(n: u64, loc: Loc) -> Self {
    Self::new(AstKind::Num(n), loc)
  }

  fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
    Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
  }

  fn binop(op: BinOp, l: Ast, r: Ast, loc: Loc) -> Self {
    Self::new(
      AstKind::BinOp {
        op,
        l: Box::new(l),
        r: Box::new(r),
      },
      loc,
    )
  }
}