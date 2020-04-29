use super::{Loc, Annot};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniOpKind {
  Plus,
  Minus,
}

pub type UniOp = Annot<UniOpKind>;

impl UniOp {
  pub fn plus(loc: Loc) -> Self {
    Self::new(UniOpKind::Plus, loc)
  }

  pub fn minus(loc: Loc) -> Self {
    Self::new(UniOpKind::Minus, loc)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOpKind {
  Add,
  Sub,
  Mul,
  Div,
  Lt,
  Equal,
  Gt,
  And,
  Or,
}

pub type BinOp = Annot<BinOpKind>;

impl BinOp {
  pub fn add(loc: Loc) -> Self {
    Self::new(BinOpKind::Add, loc)
  }
  pub fn sub(loc: Loc) -> Self {
    Self::new(BinOpKind::Sub, loc)
  }
  pub fn mul(loc: Loc) -> Self {
    Self::new(BinOpKind::Mul, loc)
  }
  pub fn div(loc: Loc) -> Self {
    Self::new(BinOpKind::Div, loc)
  }
  pub fn lt(loc: Loc) -> Self {
    Self::new(BinOpKind::Lt, loc)
  }
  pub fn equal(loc: Loc) -> Self {
    Self::new(BinOpKind::Equal, loc)
  }
  pub fn gt(loc: Loc) -> Self {
    Self::new(BinOpKind::Gt, loc)
  }
  pub fn and(loc: Loc) -> Self {
    Self::new(BinOpKind::And, loc)
  }
  pub fn or(loc: Loc) -> Self {
    Self::new(BinOpKind::Or, loc)
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
  pub fn num(n: u64, loc: Loc) -> Self {
    Self::new(AstKind::Num(n), loc)
  }

  pub fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
    Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
  }

  pub fn binop(op: BinOp, l: Ast, r: Ast, loc: Loc) -> Self {
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