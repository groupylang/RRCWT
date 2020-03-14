use super::VirtualMachine;
use libloading::Library;

#[cfg(target_os = "windows")]
use super::windows::dynamic_link;
#[cfg(target_os = "linux")]
use super::linux::dynamic_link;

struct NativeInterface {
  libs: Vec<Library>
}

impl VirtualMachine {
  pub fn jit(&self, name: &str, base_address: usize, size: usize) {
    let mut ni = NativeInterface {
      libs: Vec::with_capacity(32)
    };
    // compile
    let mut compiled = String::new();
    compiled += "#include <stdint.h>\n";
    compiled += "#include <stdlib.h>\n";
    compiled += &format!("uint64_t *registers = (uint64_t *) {};\n", (&self.registers[0] as *const i32 as u64).to_string());
    compiled += &format!("uint64_t *stack = (uint64_t *) {};\n", (&self.stack[0] as *const i32 as u64).to_string());
    compiled += &format!("uint64_t *heap = (uint64_t *) {};\n", (&self.heap[0] as *const i32 as u64).to_string());
    compiled += &format!("uint64_t *stack_pointer = (uint64_t *) {};\n", self.stack_pointer as u64);
    compiled += &format!("uint64_t *base_pointer = (uint64_t *) {};\n", self.base_pointer as u64);
    compiled += &format!("void {}(void) {{\n", name);
    for _i in base_address .. base_address + size {
      let pc = &self.text.get(_i).expect("error | Crash");
      compiled.push_str(match pc.code {
        0x00 /* NOP   */ => String::new(),
        0x02 /* BP    */ => String::new(),
        0x04 /* STORE */ => format!("\tstack[base_pointer + {}] = registers[{}];\n", pc.op0, pc.op1),
        0x05 /* LOAD  */ => format!("\tregisters[{}] = stack[base_pointer + {}];\n", pc.op1, pc.op0),
        0x06 /* PUSH  */ => String::new(), // TODO
        0x07 /* POP   */ => String::new(), // TODO
        0x10 /* ADDR  */ => format!("\tregisters[{}] = registers[{}] + registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x11 /* SUBR  */ => format!("\tregisters[{}] = registers[{}] - registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x12 /* MULR  */ => format!("\tregisters[{}] = registers[{}] * registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x13 /* DIVR  */ => format!("\tregisters[{}] = registers[{}] / registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x14 /* GT    */ => format!("\tregisters[{}] = registers[{}] > registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x15 /* GE    */ => format!("\tregisters[{}] = registers[{}] >= registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x16 /* EQ    */ => format!("\tregisters[{}] = registers[{}] == registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x18 /* AND   */ => format!("\tregisters[{}] = registers[{}] & registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x19 /* OR    */ => format!("\tregisters[{}] = registers[{}] | registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x1a /* NOT   */ => format!("\tregisters[{}] = !registers[{}];\n", pc.op0, pc.op1),
        0x1c /* SHL   */ => format!("\tregisters[{}] = registers[{}] << registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x1d /* SHR   */ => format!("\tregisters[{}] = registers[{}] >> registers[{}];\n", pc.op0, pc.op1, pc.op2),
        0x20 /* ADDI  */ => format!("\tregisters[{}] = registers[{}] + {};\n", pc.op0, pc.op1, pc.op2),
        0x21 /* SUBI  */ => format!("\tregisters[{}] = registers[{}] - {};\n", pc.op0, pc.op1, pc.op2),
        0x22 /* MULI  */ => format!("\tregisters[{}] = registers[{}] * {};\n", pc.op0, pc.op1, pc.op2),
        0x23 /* DIVI  */ => format!("\tregisters[{}] = registers[{}] / {};\n", pc.op0, pc.op1, pc.op2),
        0x40 /* GOTO  */ => String::new(), // TODO
        0x41 /* EXIT  */ => String::from("  exit(1);\n"),
        0x42 /* CALL  */ => String::new(), // TODO
        0x43 /* RET   */ => String::from("  return;\n"),
        0x44 /* IFGT  */ => String::new(), // TODO
        0x45 /* IFGE  */ => String::new(), // TODO
        0x46 /* IFEQ  */ => String::new(), // TODO
        0x50 /* NEW   */ => String::new(), // TODO
        0x51 /* SET   */ => format!("\theap[registers[{}] + {}] = registers[{}];\n", pc.op1, pc.op2, pc.op0),
        0x52 /* GET   */ => format!("\tregisters[{}] = heap[registers[{}] + {}];\n", pc.op0, pc.op1, pc.op2),
        _ => String::new() // TODO
      }.as_str());
    }
    compiled += "}";
    match dynamic_link(name, compiled.as_str()) {
      Ok(lib) => {
        ni.libs.push(lib);
      },
      Err(msg) => println!("{}: {}", msg, name)
    };
  }
  pub unsafe fn jit_exec() {}
}