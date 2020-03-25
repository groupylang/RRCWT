//! TODO comment
use super::ll::Scanner;
use super::VirtualMachine;
use super::Procedure;
use std::io::{Write, BufWriter};
use std::fs::File;
use std::process::Command;
use libloading::{Library, Symbol};
use std::ffi::CStr;

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
  #[no_mangle]
  pub fn jit(&mut self, pc: *const u32, jit_str: *const i8) {
    let file_c = &format!("../tmp/jit{}.c", pc as usize);
    let file_dll = &format!("../tmp/jit{}.dll", pc as usize);
    // compile
    let mut writer = BufWriter::new(File::create(file_c).expect(&format!("error | FileNotFound: {}", file_c)));
    writer.write("#include <windows.h>\nBOOL APIENTRY DllMain(HANDLE h, DWORD d, LPVOID l) {\n\treturn TRUE;\n}\n".as_bytes()).unwrap();
    writer.write_all(unsafe { CStr::from_ptr(jit_str) }.to_bytes()).unwrap();
    Command::new("clang")
      .args(&[file_c, "-o", file_dll, "-Wall", "-g", "-shared", "-fPIC"])
      .spawn().unwrap();
    // load dll
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
  #[no_mangle]
  pub fn jit(&mut self, pc: *const u32, jit_str: *const i8) {
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
}