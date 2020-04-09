use super::{Annot, UniOp, BinOp, Ast};
use super::error::print_annot;

pub struct Interpreter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
  DivisionByZero,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl Interpreter {
  pub fn new() -> Self {
    Interpreter
  }

  pub fn eval(&mut self, expr: &Ast) -> Result<i64, InterpreterError> {
    use super::AstKind::*;
    match expr.value {
      Num(n) => Ok(n as i64),
      UniOp { ref op, ref e } => {
        let e = self.eval(e)?;
        Ok(self.eval_uniop(op, e))
      }
      BinOp { ref op, ref l, ref r, } => {
        let l = self.eval(l)?;
        let r = self.eval(r)?;
        self.eval_binop(op, l, r)
            .map_err(|e| InterpreterError::new(e, expr.loc.clone()))
      }
    }
  }

  fn eval_uniop(&mut self, op: &UniOp, n: i64) -> i64 {
    use super::UniOpKind::*;
    match op.value {
      Plus => n,
      Minus => -n,
    }
  }
  fn eval_binop(&mut self, op: &BinOp, l: i64, r: i64) -> Result<i64, InterpreterErrorKind> {
    use super::BinOpKind::*;
    match op.value {
      Add => Ok(l + r),
      Sub => Ok(l - r),
      Mul => Ok(l * r),
      Div => {
        if r == 0 {
          Err(InterpreterErrorKind::DivisionByZero)
        } else {
          Ok(l / r)
        }
      },
      Lt => Ok(if l < r {1} else {0}),
      Equal => Ok(if l == r {1} else {0}),
      Gt => Ok(if l > r {1} else {0}),
      And => Ok(l & r),
      Or => Ok(l | r),
    }
  }
}

impl InterpreterError {
  pub fn show_diagnostic(&self, input: &str) {
    eprintln!("{}", self);
    print_annot(input, self.loc.clone());
  }
}