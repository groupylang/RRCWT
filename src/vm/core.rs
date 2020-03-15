//! TODO comment
use super::ll::Scanner;
use super::VirtualMachine;

impl VirtualMachine {
  pub fn scan(file_name: &str) -> VirtualMachine {
    let mut scanner = Scanner::new();
    scanner.initialize();
    scanner.load(file_name);
    scanner.setup()
  }
  pub fn execute(&mut self) {
    #[link(name="core")]
    extern "C" {
      fn v_exec(text: *const u8, data: *const u8, entry_point: u32);
    }
    unsafe {
      v_exec((*self.text).as_ptr(), (*self.data).as_ptr(), self.program_counter as u32);
    }
  }
  #[no_mangle]
  pub fn is_hot(&mut self, pc: *const u32) -> u8 {
    match self.hot_spots.get(&(pc as usize)) {
      Some(count) if *count > 3 => {
        1
      },
      Some(count) => {
        self.hot_spots.insert(pc as usize, *count + 1);
        0
      },
      None => {
        self.hot_spots.insert(pc as usize, 1);
        0
      }
    }
  }

}