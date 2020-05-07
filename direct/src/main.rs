//! direct invocation of RCWT virtual machine
extern crate serde;

use serde::{Deserialize, Serialize};

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::time::Instant;
use std::ffi::{CString, CStr};

/// virtual environment including registers, stack, heap, etc
#[allow(non_camel_case_types)]
enum env {}

/// information for direct invocation of virtual machine
#[derive(Serialize, Deserialize, Debug)]
struct Direct {
  num_registers: Option<u32>,
  entry_point: Option<u32>,
  natives: Option<Vec<(usize, String)>>,
  text: Vec<(u8, u8, u8, u8)>,
  data: Option<Vec<String>>,
}

fn main() -> io::Result<()> {
  fs::create_dir_all("tmp")?;
  let reader = BufReader::new(File::open("direct.yml")?);
  let direct: Direct = serde_yaml::from_reader(reader).expect("error | InvalidYamlFile");

  extern "C" {
    /// @C env* env_new(uint8_t* text, uint8_t* data, uint32_t numRegisters)
    fn env_new(text: *const u8, data: *const u8, numRegisters: u32) -> *const env;
    /// @C void native_load(env*, size_t, const char*);
    fn native_load_wrapper(e: *const env, index: usize, path: *const i8);
    /// @C uint8_t virtual_execute_wrapper(env*, uint32_t, uint32_t, uint32_t, uint32_t);
    fn virtual_execute_wrapper(e: *const env, text_size: u32, data_size: u32, numRegisters: u32, entry_point: u32) -> u8;
  }

  let mut data = Vec::new();
  if let Some(vec_data) = direct.data {
    for str_data in vec_data {
      for byte in str_data.as_bytes() {
        data.push(*byte);
      }
      data.push(0); // string in C should be terminated by null-byte
    }
  }

  // allocate and initialize memory
  let e = unsafe { env_new(
    /* text          */ direct.text.as_ptr() as *const u8,
    /* data          */ data.as_ptr() as *const u8,
    /* num registers */ direct.num_registers.unwrap_or(16)
  )};

  // load natives
  if let Some(vec_natives) = direct.natives {
    for (index, path) in vec_natives {
      let str_path: &str = &path;
      unsafe { native_load_wrapper(e, index, CString::new(str_path).unwrap().as_ptr()); }
    }
  }

  println!("[*] VMEntry");

  let timer = Instant::now();

  let status = unsafe { virtual_execute_wrapper(
      /* e             */ e,
      /* text size     */ (direct.text.len() * 4) as u32,
      /* data size     */ data.len() as u32,
      /* num registers */ direct.num_registers.unwrap_or(16),
      /* entry point   */ direct.entry_point.and_then(|x| Some(x * 4)).unwrap_or(0)
  )};

  let time = timer.elapsed();

  println!();
  println!("[*] VMExitWithStatus: {}", status);
  println!("[*] VMExecutionTime: {}.{:03} sec", time.as_secs(), time.subsec_nanos() / 1_000_000);
  Ok(())
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
pub fn print_float(arg: f32) {
  print!("{}", arg);
}