pub mod ll;
pub mod core;

use std::collections::HashMap;

#[repr(C)]
struct env {
  text: *const u8,
  data: *const u8,
  registers: *const u32,
  stack: *const u32,
  heap: *const u32,
  stack_pointer: u32,
  base_pointer: u32
}

type Procedure = extern fn(*const env) -> ();

#[derive(Default)]
#[repr(C)]
pub struct VirtualMachine {
  hot_spots: HashMap<usize, u32>,
  procedures: HashMap<usize, Procedure>,
  text: Box<[u8]>,
  data: Box<[u8]>,
  program_counter: usize,
}