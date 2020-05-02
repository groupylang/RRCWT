extern crate serde;

use serde::{Deserialize, Serialize};

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::time::Instant;
use std::ffi::CStr;

#[allow(non_camel_case_types)]
enum env {}

#[derive(Serialize, Deserialize, Debug)]
struct Direct {
  num_registers: Option<u32>,
  entry_point: Option<u32>,
  text: Vec<(u8, u8, u8, u8)>,
  data: Vec<String>,
}

fn main() -> io::Result<()> {
  fs::create_dir_all("tmp")?;
  let reader = BufReader::new(File::open("direct.yml")?);
  let direct: Direct = serde_yaml::from_reader(reader).expect("error | InvalidYamlFile");

  extern "C" {
    // env* env_new(uint8_t*, uint8_t*, uint32_t);
    fn env_new(text: *const u8, data: *const u8, numRegisters: u32) -> *const env;
    // uint8_t virtual_execute_wrapper(env*, uint32_t, uint32_t, uint32_t, uint32_t)
    fn virtual_execute_wrapper(e: *const env, text_size: u32, data_size: u32, num_registers: u32, entry_point: u32) -> u8;
  }

  let mut data = Vec::new();
  for str_data in direct.data {
    for byte in str_data.as_bytes() {
      data.push(*byte);
    }
    data.push(0);
  }

  println!("[*] VMEntry");

  let e = unsafe { env_new(
    /* text      */ direct.text.as_ptr() as *const u8,
    /* data      */ data.as_ptr() as *const u8,
    /* registers */ direct.num_registers.unwrap_or(16)
  )};

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