pub struct BasicBlock<'a> {
  pub label: &'a str,
  pub block: Vec<Instruction<'a>>
}

impl <'a> BasicBlock<'a> {
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
pub enum Condition {
  GT { op1: i8, op2: i8 },
  GE { op1: i8, op2: i8 },
  EQ { op1: i8, op2: i8 },
  TRUE,
  FALSE
}

#[allow(dead_code)]
pub enum InstrKind {
  RRR { op0: i8, op1: i8, op2: i8 },
  RRI { op0: i8, op1: i8, op2: i8 },
  // store load
  IF { cond: Condition, dst: String },
  NEW { op0: i8, size: i16 },
  R { op0: i8 }
}

pub struct Instruction<'a> {
  pub code: u8,
  pub kind: InstrKind,
  pub mnemonic: &'a str
}

impl <'a> Instruction<'a> {
  pub fn addi(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x20,
      kind: InstrKind::RRI { op0: op0, op1: op1, op2: op2 },
      mnemonic: "addi"
    }
  }
  pub fn subi(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x21,
      kind: InstrKind::RRI { op0: op0, op1: op1, op2: op2 },
      mnemonic: "subi"
    }
  }
  pub fn addr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x10,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "addr"
    }
  }
  pub fn subr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x11,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "subr"
    }
  }
  pub fn mulr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x12,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "mulr"
    }
  }
  pub fn divr(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x13,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "divr"
    }
  }
  pub fn gt(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x14,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "gt"
    }
  }
  pub fn eq(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x16,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "eq"
    }
  }
  pub fn and(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x18,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "and"
    }
  }
  pub fn or(op0: i8, op1: i8, op2: i8) -> Self {
    Instruction {
      code: 0x19,
      kind: InstrKind::RRR { op0: op0, op1: op1, op2: op2 },
      mnemonic: "or"
    }
  }
  pub fn exit(op0: i8) -> Self {
    Instruction {
      code: 0x41,
      kind: InstrKind::R { op0: op0 },
      mnemonic: "exit"
    }
  }
  pub fn iout(op0: i8) -> Self {
    Instruction {
      code: 0xfe,
      kind: InstrKind::R { op0: op0 },
      mnemonic: "iout"
    }
  }
  pub fn gen(&self, buf: &mut Vec<u8>) {
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