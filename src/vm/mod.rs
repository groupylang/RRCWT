pub mod ll;
pub mod core;

#[derive(Default)]
pub struct VirtualMachine {
  text: Box<[u8]>,
  data: Box<[u8]>,
  program_counter: usize,
}