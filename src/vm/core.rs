//! TODO comment
use super::ll::Scanner;
use super::VirtualMachine;
use super::Procedure;
use std::io::{Write, BufWriter};
use std::fs::File;
use std::process::Command;
use libloading::{Library, Symbol};

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
      fn v_exec(vm: *const VirtualMachine, text: *const u8, data: *const u8, entry_point: u32);
    }
    unsafe {
      v_exec(self, (*self.text).as_ptr(), (*self.data).as_ptr(), self.program_counter as u32);
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
  #[cfg(target_os="windows")]
  #[no_mangle]
  pub fn jit(&mut self, pc: *const u32, jit_str: *const i8) {
    use std::ffi::CStr;
    // compile
    let mut writer = BufWriter::new(File::create("tmp/f.c").expect("error | FileNotFound: f.c"));
    writer.write("#include <windows.h>\nBOOL APIENTRY DllMain(HANDLE h, DWORD d, LPVOID l) {\n\treturn TRUE;\n}\n".as_bytes()).unwrap();
    writer.write_all(unsafe { CStr::from_ptr(jit_str) }.to_bytes()).unwrap();
    Command::new("clang")
      .args(&["tmp/f.c", "-o", "tmp/f.dll", "-Wall", "-g", "-shared", "-fPIC"])
      .spawn().unwrap();
    // load dll
    match Library::new("tmp/f.dll") {
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
        println!("{}: {}", msg, "tmp/f.dll")
      }
    }
  }
  #[cfg(target_os="linux")]
  #[no_mangle]
  pub fn jit() {
    use std::ffi::CStr;
    // compile
    let mut writer = BufWriter::new(File::create("tmp/f.c").expect("error | FileNotFound: f.c"));
    writer.write_all(unsafe { CStr::from_ptr(jit_str) }.to_bytes()).unwrap();
    Command::new("clang")
      .args(&["tmp/f.c", "-o", "tmp/f.so", "-Wall", "-g", "-shared", "-fPIC"])
      .spawn().unwrap();
    // load so
    match Library::new("tmp/f.so") {
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
        println!("{}: {}", msg, "tmp/f.so")
      }
    }
  }
}