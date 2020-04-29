use super::ast::Ast;
use super::ir::{BasicBlock, Instruction};

pub struct IrCompiler {
  tmp_count: i8 // TODO count of using registers
}

impl IrCompiler {
  pub fn new() -> Self {
    IrCompiler { tmp_count: 0 }
  }

  pub fn compile(&mut self, expr: &Ast) -> BasicBlock {
    let mut block = Vec::with_capacity(32);
    let _expr = self.compile_inner(expr, &mut block);
    block.push(Instruction::iout(_expr));
    block.push(Instruction::exit(0x00));
    BasicBlock { label: "main", block }
  }

  fn compile_inner(&mut self, expr: &Ast, block: &mut Vec<Instruction>) -> i8 {
    use super::ast::AstKind::*;
    match expr.value {
      Num(n) => {
        self.tmp_count += 1;
        block.push(Instruction::addi(self.tmp_count, 0x00, n as i8));
        self.tmp_count
      },
      UniOp { ref op, ref e } => {
        let _e = self.compile_inner(e, block);
        self.tmp_count += 1;
        use super::ast::UniOpKind::*;
        match op.value {
          Plus => block.push(Instruction::addi(self.tmp_count, 0x00, _e)),
          Minus => block.push(Instruction::subi(self.tmp_count, 0x00, _e)),
        }
        self.tmp_count
      }
      BinOp { ref op, ref l, ref r } => {
        let _l = self.compile_inner(l, block);
        let _r = self.compile_inner(r, block);
        self.tmp_count += 1;
        use super::ast::BinOpKind::*;
        match op.value {
          Add => block.push(Instruction::addr(self.tmp_count, _l, _r)),
          Sub => block.push(Instruction::subr(self.tmp_count, _l, _r)),
          Mul => block.push(Instruction::mulr(self.tmp_count, _l, _r)),
          Div => block.push(Instruction::divr(self.tmp_count, _l, _r)),
          Gt => block.push(Instruction::gt(self.tmp_count, _l, _r)),
          Equal => block.push(Instruction::eq(self.tmp_count, _l, _r)),
          Lt => block.push(Instruction::gt(self.tmp_count, _r, _l)),
          And => block.push(Instruction::and(self.tmp_count, _l, _r)),
          Or  => block.push(Instruction::or(self.tmp_count, _l, _r)),
        }
        self.tmp_count
      }
    }
  }
}