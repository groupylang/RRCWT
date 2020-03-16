use super::{UniOpKind, UniOp, BinOpKind, BinOp, AstKind, Ast};

pub struct IrCompiler {
  tmp_count: isize
}

impl IrCompiler {
  pub fn new() -> Self {
    IrCompiler { tmp_count: 0 }
  }

  pub fn compile(&mut self, expr: &Ast) -> String {
    let mut buf = String::new();
    let _expr = self.compile_inner(expr, &mut buf);
    buf.push_str(&_expr);
    buf
  }

  fn compile_inner(&mut self, expr: &Ast, buf: &mut String) -> String {
    use self::AstKind::*;
    match expr.value {
      Num(n) => n.to_string(),
      UniOp { ref op, ref e } => {
        let _e = self.compile_inner(e, buf);
        self.tmp_count += 1;
        let tmp_name = format!("{}{}", '$', self.tmp_count.to_string());
        buf.push_str(&tmp_name);
        buf.push('=');
        buf.push_str(self.compile_uniop(op));
        buf.push_str(&_e);
        buf.push('\n');
        tmp_name
      }
      BinOp { ref op, ref l, ref r, } => {
        let _l = self.compile_inner(l, buf);
        let _r = self.compile_inner(r, buf);
        self.tmp_count += 1;
        let tmp_name = format!("{}{}", '$', self.tmp_count.to_string());
        buf.push_str(&tmp_name);
        buf.push_str("=");
        buf.push_str(&_l);
        buf.push_str(self.compile_binop(op));
        buf.push_str(&_r);
        buf.push_str("\n");
        tmp_name
      }
    }
  }

  fn compile_uniop(&mut self, op: &UniOp) -> &str {
    use self::UniOpKind::*;
    match op.value {
      Plus => "+",
      Minus => "-",
    }
  }
  fn compile_binop(&mut self, op: &BinOp) -> &str {
    use self::BinOpKind::*;
    match op.value {
      Add => "+",
      Sub => "-",
      Mult => "*",
      Div => "/",
    }
  }
}