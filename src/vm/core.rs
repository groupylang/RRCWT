//! TODO comment
use super::ll::Scanner;
use super::VirtualMachine;

impl VirtualMachine {
  pub fn scan(file_name: &str) -> VirtualMachine {
    let mut scanner = Scanner::new();
    scanner.initialize();
    scanner.load(file_name);
    scanner.setup()
  }
  // @before scan
  pub fn execute(&mut self) {
    // initialize
    self.stack = Vec::with_capacity(32);
    self.heap = Vec::with_capacity(32);
    self.registers = vec![0; 1024]; // TODO
    self.stack_pointer = 0;
    self.base_pointer = 0;
    // execute
    loop {
      let i = self.text.get(self.program_counter).expect("error | Crash");
      match i.code {
        0x00 /* NOP   */ => (),
        0x02 /* BP    */ => println!("debug | BreakPoint: {:<02X}, {:<02X}, {:<02X}", i.operand0, i.operand1, i.operand2),
        0x04 /* STORE */ => *self.stack_as_box(self.base_pointer as i32 + i.operand0 as i32) = self.reg(i.operand1),
        0x05 /* LOAD  */ => *self.reg_as_box(i.operand1) = self.stack(self.base_pointer as i32 + i.operand0 as i32),
        0x06 /* PUSH  */ => self.stack.push(self.reg(i.operand0)),
        0x07 /* POP   */ => *self.reg_as_box(i.operand0) = self.stack.pop().expect("error | StackEmpty"),
        0x10 /* ADDR  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) + self.reg(i.operand2),
        0x11 /* SUBR  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) - self.reg(i.operand2),
        0x12 /* MULR  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) * self.reg(i.operand2),
        0x13 /* DIVR  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) / self.reg(i.operand2),
        0x14 /* GT    */ => *self.reg_as_box(i.operand0) =
                              if self.reg(i.operand1) > self.reg(i.operand2) { 0x00000001 }
                              else { 0x00000000 },
        0x15 /* GE    */ => *self.reg_as_box(i.operand0) =
                              if self.reg(i.operand1) >= self.reg(i.operand2) { 0x00000001 }
                              else { 0x00000000 },
        0x16 /* EQ    */ => *self.reg_as_box(i.operand0) =
                              if self.reg(i.operand1) == self.reg(i.operand2) { 0x00000001 }
                              else { 0x00000000 },
        0x18 /* AND   */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) & self.reg(i.operand2),
        0x19 /* OR    */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) | self.reg(i.operand2),
        0x1a /* NOT   */ => *self.reg_as_box(i.operand0) = !self.reg(i.operand1),
        0x1c /* SHL   */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) << self.reg(i.operand2),
        0x1d /* SHR   */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) >> self.reg(i.operand2),
        0x20 /* ADDI  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) + i.operand2 as i32,
        0x21 /* SUBI  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) - i.operand2 as i32,
        0x22 /* MULI  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) * i.operand2 as i32,
        0x23 /* DIVI  */ => *self.reg_as_box(i.operand0) = self.reg(i.operand1) / i.operand2 as i32,
        0x40 /* GOTO  */ => {
          self.program_counter = (self.program_counter as isize + i.operand0 as isize) as usize; continue;
        },
        0x41 /* EXIT  */ => return,
        0x42 /* CALL  */ => {
          self.stack.push(self.base_pointer as i32);
          self.base_pointer = self.stack_pointer;
          self.stack.resize(self.stack.len() + i.operand0 as usize, 0);
          self.stack.push(self.program_counter as i32);
          self.program_counter = i.operand2 as usize / 4; // TODO
          continue;
          // jit(base, size)
        },
        0x43 /* RET   */ => {
          self.program_counter = self.stack.pop().expect("error | StackEmpty") as usize;
          self.stack_pointer = self.base_pointer;
          self.base_pointer = self.stack.pop().expect("error | StackEmpty") as usize;
        },
        0x44 /* IFGT  */ => if self.reg(i.operand1) > self.reg(i.operand2)
                              { self.program_counter = (self.program_counter as isize + i.operand0 as isize) as usize; continue; },
        0x45 /* IFGE  */ => if self.reg(i.operand1) >= self.reg(i.operand2)
                              { self.program_counter = (self.program_counter as isize + i.operand0 as isize) as usize; continue; },
        0x46 /* IFEQ  */ => if self.reg(i.operand1) == self.reg(i.operand2)
                              { self.program_counter = (self.program_counter as isize + i.operand0 as isize) as usize; continue; },

        0x50 /* NEW   */ => {
          if i.operand1 == 0 { println!("warning | InstanceZeroSized"); }
          let base = self.heap.len();
          for _ in 0 .. i.operand1 { self.heap.push(0) }
          *self.reg_as_box(i.operand0) = base as i32;
        },
        0x51 /* SET   */ => *self.heap_as_box(self.registers[i.operand1 as usize] as usize + i.operand2 as usize) = self.reg(i.operand0),
        0x52 /* GET   */ => *self.reg_as_box(i.operand0) = self.heap(self.reg(i.operand1) as usize + i.operand2 as usize),
        0xff /* SOUT  */ => {
          let mut _i = self.reg(i.operand0) as usize;
          loop {
            let c = *self.data.get(_i).expect("error | IndexOutOfBounds: data");
            if c == 0 { break; }
            print!("{}", char::from(c));
            _i += 1;
          }
        },
        _ => println!("warning | InvalidInstruction")
      };
      self.program_counter += 1;
    }
  }
  fn reg_as_box(&self, index: i8) -> Box<i32> {
    Box::new(*self.registers.get(index as usize).expect("error | IndexOutOfBounds: registers"))
  }
  fn reg(&self, index: i8) -> i32 {
    *self.registers.get(index as usize).expect("error | IndexOutOfBounds: registers")
  }
  fn stack_as_box(&self, index: i32) -> Box<i32> {
    Box::new(*self.stack.get(index as usize).expect("error | IndexOutOfBounds: stack"))
  }
  fn stack(&self, index: i32) -> i32 {
    *self.stack.get(index as usize).expect("error | IndexOutOfBounds: stack")
  }
  fn heap_as_box(&self, index: usize) -> Box<i32> {
    Box::new(*self.heap.get(index).expect("error | IndexOutOfBounds: heap"))
  }
  fn heap(&self, index: usize) -> i32 {
    *self.heap.get(index).expect("error | IndexOutOfBounds: heap")
  }
}

#[cfg(test)]
mod tests {
  
}