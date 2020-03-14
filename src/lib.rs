extern crate libloading;

pub mod vm;

#[derive(Default)]
pub struct Instruction {
  code: u8,
  op0: i8,
  op1: i8,
  op2: i8
}

// #[cfg(target_os = "linux")]
// extern crate llvm_sys;
// #[cfg(target_os = "linux")]
// extern crate rustc_llvm_proxy;