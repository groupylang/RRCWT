use super::{Ast};
use std::fmt;

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
    BasicBlock { label: String::from("main"), block }
  }

  fn compile_inner(&mut self, expr: &Ast, block: &mut Vec<Instruction>) -> i8 {
    use super::AstKind::*;
    match expr.value {
      Num(n) => {
        self.tmp_count += 1;
        block.push(Instruction::addi(self.tmp_count, 0x00, n as i8));
        self.tmp_count
      },
      UniOp { ref op, ref e } => {
        let _e = self.compile_inner(e, block);
        self.tmp_count += 1;
        use super::UniOpKind::*;
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
        use super::BinOpKind::*;
        match op.value {
          Add => block.push(Instruction::addr(self.tmp_count, _l, _r)),
          Sub => block.push(Instruction::subr(self.tmp_count, _l, _r)),
          Mult => block.push(Instruction::mulr(self.tmp_count, _l, _r)),
          Div => block.push(Instruction::divr(self.tmp_count, _l, _r)),
        }
        self.tmp_count
      }
    }
  }
}

pub struct BasicBlock {
  label: String,
  block: Vec<Instruction>
}

impl fmt::Debug for BasicBlock {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let result = writeln!(f, "{}:", self.label);
    for instr in &self.block {
      writeln!(f, "{:?}", instr).unwrap();
    }
    result
  }
}

impl fmt::Display for BasicBlock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result = writeln!(f, "{}:", self.label);
    for instr in &self.block {
      writeln!(f, "{}", instr).unwrap();
    }
    result
  }
}

impl BasicBlock {
  pub fn gen(&self) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(32);
    buf.push(0x00); buf.push(4 * self.block.len() as u8);
    buf.push(0x00); buf.push(0x00);
    buf.push(0x00); buf.push(0x01);
    buf.push(0x00); buf.push(0x00);
    buf.push(0x00); buf.push(0x00);
    buf.push(0x6D); buf.push(0x61); buf.push(0x69); buf.push(0x6E); buf.push(0x00);
    buf.push(0x00); buf.push(0x00);
    buf.push(0x00); buf.push(0x00);
    for instr in &self.block {
      instr.gen(&mut buf);
    }
    buf
  }
}
#[allow(dead_code)]
enum Condition {
  GT { op1: i8, op2: i8 },
  GE { op1: i8, op2: i8 },
  EQ { op1: i8, op2: i8 },
  TRUE,
  FALSE
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

#[allow(dead_code)]
enum InstrKind {
  RRR { op0: i8, op1: i8, op2: i8 },
  RRI { op0: i8, op1: i8, op2: i8 },
  // store load
  IF { cond: Condition, dst: String },
  NEW { op0: i8, size: i16 },
  R { op0: i8 }
}

struct Instruction {
  code: u8,
  kind: InstrKind,
  mnemonic: String
}

impl fmt::Debug for Instruction {
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

impl fmt::Display for Instruction {
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
impl Instruction {
  fn addi(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x20,
      kind: InstrKind::RRI { op0: op0, op1: op1, op2: op2},
      mnemonic: String::from("addi")
    }
  }
  fn subi(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x21,
      kind: InstrKind::RRI { op0: op0, op1: op1, op2: op2},
      mnemonic: String::from("subi")
    }
  }
  fn addr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x10,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2},
      mnemonic: String::from("addr")
    }
  }
  fn subr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x11,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2},
      mnemonic: String::from("subr")
    }
  }
  fn mulr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x12,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2},
      mnemonic: String::from("mulr")
    }
  }
  fn divr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x13,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2},
      mnemonic: String::from("divr")
    }
  }
  fn exit(op0: i8) -> Self {
    Instruction {
      code: 0x41,
      kind: InstrKind::R { op0: op0 },
      mnemonic: String::from("exit")
    }
  }
  fn iout(op0: i8) -> Self {
    Instruction {
      code: 0xfe,
      kind: InstrKind::R { op0: op0 },
      mnemonic: String::from("iout")
    }
  }
  fn gen(&self, buf: &mut Vec<u8>) {
    use InstrKind::*;
    match &self.kind {
      RRR { op0, op1, op2 } => {
        buf.push(self.code); buf.push(*op0 as u8); buf.push(*op1 as u8); buf.push(*op2 as u8);
      },
      RRI { op0, op1, op2 } => {
        buf.push(self.code); buf.push(*op0 as u8); buf.push(*op1 as u8); buf.push(*op2 as u8);
      },
      #[allow(unused_variables)]
      IF { cond, dst } => {
        use Condition::*;
        match cond {
          GT { op1, op2 } => {
            buf.push(0x44); buf.push(0x00); buf.push(*op1 as u8); buf.push(*op2 as u8); // TODO op0
          },
          GE { op1, op2 } => {
            buf.push(0x45); buf.push(0x00); buf.push(*op1 as u8); buf.push(*op2 as u8); // TODO op0
          },
          EQ { op1, op2 } => {
            buf.push(0x46); buf.push(0x00); buf.push(*op1 as u8); buf.push(*op2 as u8); // TODO op0
          },
          TRUE  => {
            buf.push(0x40); buf.push(0x00); buf.push(0x00); buf.push(0x00); // TODO op0
          },
          FALSE => {
            buf.push(0x00); buf.push(0x00); buf.push(0x00); buf.push(0x00); // TODO op0
          }
        }
      },
      NEW { op0, size } => {
        buf.push(0x50); buf.push(*op0 as u8);
        buf.push(((size >> 8) & 0xffi16) as u8); buf.push((size & 0xffi16) as u8);
      },
      R { op0 } => {
        buf.push(self.code); buf.push(*op0 as u8); buf.push(0x00); buf.push(0x00);
      }
      // _ => println!("error | InvalidInstrKind")
    }
  }
}