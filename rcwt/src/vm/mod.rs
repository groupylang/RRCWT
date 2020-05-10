pub mod ll;
pub mod core;

use std::collections::HashMap;

/// virtual environment including registers, stack, heap, etc
#[allow(non_camel_case_types)]
pub enum env {}

/// native procedure to be loaded and invoked by virtual machine(unused)
type Procedure = extern fn(*const env) -> ();

/// interface(unused): Rust to C++
#[derive(Default)]
pub struct VirtualMachine {
  hot_spots: HashMap<usize, u32>,
  natives: HashMap<usize, Procedure>,
  text: Box<[u8]>,
  data: Box<[u8]>,
  program_counter: usize,
}