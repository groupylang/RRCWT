pub mod vm;
pub mod native;

#[derive(Default)]
pub struct Instruction {
  code: u8,
  operand0: i8,
  operand1: i8,
  operand2: i8
}

#[cfg(target_os = "linux")]
extern crate llvm_sys;
#[cfg(target_os = "linux")]
extern crate rustc_llvm_proxy;