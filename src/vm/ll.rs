//! TODO comment
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use super::VirtualMachine;
use super::super::Instruction;

macro_rules! read {
  ($($i: ident: $t: tt),*) => {
    $(let mut $i = read_inner!($t);)*
  };
}

macro_rules! read_inner {
  // ex. u32
  ($i: ident) => {
    paste::expr! {
      reader.[<read_$i>]().expect("error | InvalidFormat")
    }
  };
  // ex. (u32, u32)
  (($($t: tt),*)) => {{
    ($(read_inner!($t)),*)
  }};
  // ex. [u32; 8]
  ([$t: tt; $e: expr]) => {{
    let mut v = Vec::with_capacity($e as usize);
    for _i in 0 .. $e as usize {
      v.push(read_inner!($t));
    }
    v
  }};
  // ex. {T p: u32, q: u32}
  ({$i: ident $($j: ident: $t: tt),*}) => {{
    $(let $j; read!($j: $t);)*
    $i { $($j: $j,)* }
  }};
}

macro_rules! atoms {
  ($($i: ident: $t: tt),*) => {
    $(atoms_inner!($i: $t);)*
  };
}

macro_rules! atoms_inner {
  // ex. T { x: u32, y: u32 }
  ($i: ident: { $($j: ident: $k: ident),* }) => {
    #[derive(Default)]
    pub struct $i {
      $($j: $k),*
    }
    impl BinaryReader {
      paste::item! {
        #[allow(non_snake_case)]
        pub fn [<read_$i>](&mut self) -> Option<$i> {
          Some($i { $($j: self.[<read_$k>]().expect("error | InvalidFormat")),* })
        }
      }
    }
  };
  // ex. T: u32
  ($i: ident: $j: ident) => {
    impl BinaryReader {
      paste::item! {
        #[allow(non_snake_case)]
        pub fn [<read_$i>](&mut self) -> Option<$i> {
          self.[<read_$j>]().map(|n| n as $i)
        }
      }
    }
  };
}

atoms!(
  usize: u16,
  DefinedHeader: { symbol_name: CString, segment_id: usize, base_address: usize },
  UndefinedHeader: { symbol_name: CString, module_name: CString },
  RelocationHeader: { symbol_name: CString, segment_id: usize, base_address: usize }
);

#[derive(Default)]
pub struct Scanner {
  symbols: HashMap<String, DefinedHeader>,
  relocations: Vec<RelocationHeader>,
  text: Vec<u8>,
  data: Vec<u8>,
}
impl Scanner {
  pub fn new() -> Scanner {
    Scanner {
      .. Default::default()
    }
  }
  pub fn initialize(&mut self) {
    self.symbols = HashMap::with_capacity(32);
    self.relocations = Vec::with_capacity(32);
    self.text = Vec::with_capacity(32);
    self.data = Vec::with_capacity(32);
  }
  pub fn load(&mut self, file_name: &str) {
    let mut reader = BinaryReader::open(file_name);
    let text_offset = self.text.len();
    let data_offset = self.data.len();
    // read
    read! { magic: u32 }
    if magic != 0x52435754 {
      println!("error | InvalidFile: {}", file_name);
      return
    }
    read! {
      text_size: usize,
      data_size: usize,
      defined_count: usize,
      undefined_count: usize,
      relocation_count: usize,
      defined_table: [DefinedHeader; defined_count],
      undefined_table: [UndefinedHeader; undefined_count],
      relocations: [RelocationHeader; relocation_count],
      text: [u8; text_size],
      data: [u8; data_size]
    }
    self.text.append(&mut text);
    self.data.append(&mut data);
    // relocate
    self.relocations.append(&mut relocations.into_iter().map(|mut r| {
      match r.segment_id {
        0 => r.base_address += text_offset,
        1 => r.base_address += data_offset,
        _ => println!("warning | InvalidSegmentId in relocation header")
      };
      r
    }).collect::<Vec<RelocationHeader>>());
    defined_table.into_iter().for_each(|mut d| {
      match d.segment_id {
        0 => d.base_address += text_offset,
        1 => d.base_address += data_offset,
        _ => println!("warning | InvalidSegmentId in defined table")
      };
      self.symbols.entry(std::mem::take(&mut d.symbol_name)).or_insert(d);
    });
    // resolve
    let mut us = undefined_table;
    while !us.is_empty() {
      us[0].module_name.pop();
      self.load(&us[0].module_name);
      us = us.into_iter()
        .filter(|u| !self.symbols.contains_key(&u.symbol_name))
        .collect::<Vec<UndefinedHeader>>();
    }
  }
  pub fn setup(&mut self) -> VirtualMachine {
    // TODO verify
    let offset = self.text.len();
    let mut body = Vec::from(std::mem::take(&mut self.text));
    body.append(&mut self.data);
    // patch
    for _r in std::mem::take(&mut self.relocations) {
      let s = self.symbols.get(&_r.symbol_name).unwrap();
      match s.segment_id {
        0 => body[_r.base_address] = s.base_address as u8,
        1 => body[_r.base_address] = (s.base_address + offset) as u8,
        _ => println!("warning | InvalidSegmentId in relocation header")
      }
    }
    let two = body.split_at(offset);
    let mut tmp = Vec::with_capacity(32);
    for _i in 0 .. two.0.len() / 4 {
      tmp.push(Instruction {
        code: two.0[_i * 4],
        operand0: two.0[_i * 4 + 1] as i8,
        operand1: two.0[_i * 4 + 2] as i8,
        operand2: two.0[_i * 4 + 3] as i8
      })
    }
    VirtualMachine {
      text: tmp.into_boxed_slice(),
      data: Box::from(two.1),
      .. Default::default()
    }
  }
  pub fn get_entry_point(&mut self) -> usize {
    self.symbols.get("main\0").expect("error | EntryPointNotFound").base_address
  }
}

type CString = String;

pub struct BinaryReader {
  reader: BufReader<File>
}
impl BinaryReader {
  fn open(file_name: &str) -> BinaryReader {
    let extended = String::from(file_name) + ".wc";
    let error_message = String::from("error | FileNotFound: ") + &extended;
    BinaryReader {
      reader: BufReader::new(File::open(extended).expect(&error_message))
    }
  }
  fn read_to_end(&mut self) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(32);
    match self.reader.read_to_end(&mut buf) {
      Ok(n) if n > 0 => Some(buf),
      _ => None
    }
  }
  fn read_u8(&mut self) -> Option<u8> {
    let mut buf = [0u8; 1];
    match self.reader.read_exact(&mut buf) {
      Ok(()) => Some(buf[0]),
      Err(_) => None
    }
  }
  fn read_u16(&mut self) -> Option<u16> {
    let mut buf = [0u8; 2];
    match self.reader.read_exact(&mut buf) {
      Ok(()) => Some(
        ((buf[0] as u16) << 8)
        + buf[1] as u16
      ),
      Err(_) => None
    }
  }
  fn read_u32(&mut self) -> Option<u32> {
    let mut buf = [0u8; 4];
    match self.reader.read_exact(&mut buf) {
      Ok(()) => Some(
        ((buf[0] as u32) << 24)
        + ((buf[1] as u32) << 16)
        + ((buf[2] as u32) << 8)
        + buf[3] as u32
      ),
      Err(_) => None
    }
  }
  #[allow(non_snake_case)]
  fn read_CString(&mut self) -> Option<String> {
    let mut buf = Vec::new();
    match self.reader.read_until(b'\0', &mut buf) {
      Ok(n) if n > 0 => Some(String::from_utf8(buf).expect("error | StringInvalid")),
      _ => None
    }
  }
}

#[cfg(test)]
mod tests {
  
}