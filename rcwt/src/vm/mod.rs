pub mod ll;
pub mod core;

use std::collections::HashMap;

#[repr(C)]
pub struct env;

#[derive(Default)]
pub struct VirtualMachine {
  hot_spots: HashMap<usize, u32>,
  text: Box<[u8]>,
  data: Box<[u8]>,
  program_counter: usize,
}