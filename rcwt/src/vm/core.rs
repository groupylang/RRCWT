//! scan and execute `wc` files
use super::ll::Scanner;
use std::ffi::CStr;
use super::{env, Procedure, VirtualMachine};
use std::io::{Write, BufWriter};
use std::io;
use std::fs::File;
use std::process::Command;
use libloading::{Library, Symbol};

impl VirtualMachine {
  pub fn scan(file_name: &str) -> io::Result<VirtualMachine> {
    let mut scanner = Scanner::new();
    scanner.initialize();
    scanner.load(file_name);
    scanner.setup()
  }
  /// wrapper of virtual_execute()
  pub fn execute(&mut self) {
    #[link(name="core")]
    extern "C" {
      /// @C env* env_new(uint8_t*, uint8_t*, uint32_t);
      fn env_new(text: *const u8, data: *const u8, numRegisters: u32) -> *const env;
      /// @C uint8_t virtual_execute(uint32_t*, env*, uint32_t);
      fn virtual_execute(vm: *const u32, e: *const env, entry_point: u32) -> u8;
    }
    let status = unsafe {
      let e = env_new((*self.text).as_ptr(), (*self.data).as_ptr(), 1024);
      virtual_execute(self as *const VirtualMachine as *const u32, e, self.program_counter as u32)
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
  /// count how many times vm calls the virtual function and check if it is hot
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
  #[cfg(target_os="windows")]
  /// # Panics
  /// if impossible to write to `.c` and `.dll` file, created during jit-assembling
  #[no_mangle]
  pub fn jit_assemble(&mut self, pc: *const u32, jit_str: *const i8) {
    let file_c = &format!("../tmp/jit{}.c", pc as usize);
    let file_dll = &format!("../tmp/jit{}.dll", pc as usize);
    // compile
    let mut writer = BufWriter::new(File::create(file_c).expect(&format!("error | FileNotFound: {}", file_c)));
    writer.write("#include <windows.h>\nBOOL APIENTRY DllMain(HANDLE h, DWORD d, LPVOID l) {\n\treturn TRUE;\n}\n".as_bytes()).unwrap();
    writer.write_all(unsafe { CStr::from_ptr(jit_str) }.to_bytes()).unwrap();
    Command::new("clang")
      .args(&[file_c, "-o", file_dll, "-Wall", "-g", "-shared", "-fPIC"])
      .spawn().unwrap();
    // load so
    match Library::new(file_dll) {
      Ok(lib) => {
        let opt_procedure: Result<Symbol<Procedure>, _> = unsafe {
          lib.get("f\0".as_bytes())
        };
        match opt_procedure {
          Ok(procedure) => {
            self.procedures.insert(pc as usize, *procedure);
          }
          Err(msg) => {
            println!("{}: `{}`", msg, "f\0")
          }
        }
      }
      Err(msg) => {
        println!("{}: {}", msg, file_dll)
      }
    }
  }
  #[cfg(target_os="linux")]
  /// # Panics
  /// if impossible to write to `.c` and `.so` file, created during jit-assembling
  #[no_mangle]
  pub fn jit_assemble(&mut self, pc: *const u32, jit_str: *const i8) {
    let file_c = &format!("../tmp/jit{}.c", pc as usize);
    let file_so = &format!("../tmp/jit{}.so", pc as usize);
    // compile
    let mut writer = BufWriter::new(File::create(file_c).expect(&format!("error | FileNotFound: {}", file_c)));
    writer.write_all(unsafe { CStr::from_ptr(jit_str) }.to_bytes()).unwrap();
    Command::new("clang")
      .args(&[file_c, "-o", file_so, "-Wall", "-g", "-shared", "-fPIC"])
      .spawn().unwrap();
    // load so
    match Library::new(file_so) {
      Ok(lib) => {
        let opt_procedure: Result<Symbol<Procedure>, _> = unsafe {
          lib.get("f\0".as_bytes())
        };
        match opt_procedure {
          Ok(procedure) => {
            self.procedures.insert(pc as usize, *procedure);
          }
          Err(msg) => {
            println!("{}: `{}`", msg, "f\0")
          }
        }
      }
      Err(msg) => {
        println!("{}: {}", msg, file_so)
      }
    }
  }
  /// execute native function
  #[no_mangle]
  pub fn native_execute(&mut self, pc: *const u32, e: &env) {
    self.procedures.get(&(pc as usize)).expect(&format!("error | ProcedureNotFound: {}", pc as usize))(e);
  }
}