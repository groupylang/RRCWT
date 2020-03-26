//! TODO comment
use super::ll::Scanner;
use super::VirtualMachine;
use std::ffi::CStr;

impl VirtualMachine {
  pub fn scan(file_name: &str) -> VirtualMachine {
    let mut scanner = Scanner::new();
    scanner.initialize();
    scanner.load(file_name);
    scanner.setup()
  }
  // wrapper of v_exec()
  pub fn execute(&mut self) {
    #[link(name="core")]
    extern "C" {
      fn v_exec(vm: *const u32, text: *const u8, data: *const u8, entry_point: u32) -> u8;
    }
    let status = unsafe {
      v_exec(self as *const VirtualMachine as *const u32, (*self.text).as_ptr(), (*self.data).as_ptr(), self.program_counter as u32)
    };
    println!();
    println!("log | VMExitWithStatus: {}", status);
  }
  #[no_mangle]
  pub fn print_int(arg: u32) {
    print!("{}", arg);
  }
  #[no_mangle]
  pub fn print_str(arg: *const i8) {
    print!("{}", unsafe { CStr::from_ptr(arg) }.to_str().expect("error | PrintInvalidString"));
  }
  // count how many times vm calls the virtual function and check if it is hot
  #[no_mangle]
  pub fn is_hot(&mut self, pc: *const u32) -> u8 {
    match self.hot_spots.get(&(pc as usize)) {
      Some(count) if *count > 3 => {
        1
      },
      Some(count) => {
        #[allow(mutable_borrow_reservation_conflict)]
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