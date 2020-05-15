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
  pub fn scan(path: &str) -> io::Result<VirtualMachine> {
    let mut scanner = Scanner::new();
    scanner.initialize();
    scanner.load(path);
    scanner.setup()
  }
  /// wrapper of virtual_execute_wrapper()
  pub fn execute(&mut self) {
    #[link(name="core")]
    extern "C" {
      /// @C env* env_new(uint8_t*, uint8_t*, uint32_t);
      fn env_new(text: *const u8, data: *const u8, numRegisters: u32) -> *const env;
      /// @C uint8_t virtual_execute_wrapper(env*, uint32_t, uint32_t, uint32_t, uint32_t);
      fn virtual_execute_wrapper(e: *const env, text_size: u32, data_size: u32, num_registers: u32, entry_point: u32) -> u8;
    }
    let e = unsafe { env_new(
      /* text          */ (*self.text).as_ptr(),
      /* data          */ (*self.data).as_ptr(),
      /* num registers */ 32,
    )};
    let status = unsafe { virtual_execute_wrapper(
      /* e             */ e,
      /* text size     */ self.text.len() as u32,
      /* data size     */ self.data.len() as u32,
      /* num registers */ 32,
      /* entry point   */ self.program_counter as u32,
    )};
    println!();
    println!("log | VMExitWithStatus: {}", status);
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
            self.natives.insert(pc as usize, *procedure);
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
            self.natives.insert(pc as usize, *procedure);
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
  /// - arg `pc` identifier of native procedure = address of virtual procedure's first instruction
  #[no_mangle]
  pub fn native_execute(&mut self, pc: *const u32, e: &env) {
    self.natives.get(&(pc as usize)).expect(&format!("error | ProcedureNotFound: {}", pc as usize))(e);
  }
}