pub mod ll;
pub mod core;

use std::collections::HashMap;

#[repr(C)]
struct env;

#[derive(Default)]
#[repr(C)]
pub struct VirtualMachine {
  hot_spots: HashMap<usize, u32>,
  procedures: HashMap<usize, fn(*const env) -> ()>,
  text: Box<[u8]>,
  data: Box<[u8]>,
  program_counter: usize,
}