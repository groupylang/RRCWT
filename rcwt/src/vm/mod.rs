pub mod ll;
pub mod core;

use std::collections::HashMap;

#[allow(non_camel_case_types)]
pub enum env {}

type Procedure = extern fn(*const env) -> ();

#[derive(Default)]
pub struct VirtualMachine {
  hot_spots: HashMap<usize, u32>,
  procedures: HashMap<usize, Procedure>,
  text: Box<[u8]>,
  data: Box<[u8]>,
  program_counter: usize,
}