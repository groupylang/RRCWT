pub mod token;
pub mod ast;
pub mod ir;

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