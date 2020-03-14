pub mod ll;
pub mod core;
pub mod err;
pub mod jit;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "linux")]
pub mod linux;

use super::Instruction;

#[derive(Default)]
pub struct VirtualMachine {
  text: Box<[Instruction]>,
  data: Box<[u8]>,
  program_counter: usize,
  stack: Vec<i32>,
  heap: Vec<i32>,
  registers: Vec<i32>,
  stack_pointer: usize,
  base_pointer: usize,
}