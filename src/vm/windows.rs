use std::process::Command;
use std::io::{Write, BufWriter};
use std::fs::File;
use libloading::Library;
use std::io::Error;

pub fn dynamic_link(file_name: &str, buf: &str) -> Result<Library, Error> {
  let c = String::from("tmp/") + file_name + ".c";
  let dll = String::from("tmp/") + file_name + ".dll";
  File::create(&dll).unwrap();
  let mut writer: BufWriter<File> = BufWriter::new(File::create(&c).unwrap());
  match writer.write_all(buf.as_bytes()) {
    Ok(()) => (),
    Err(msg) => println!("{}", msg),
  }
  // assemble
  Command::new("clang")
      .args(&[&c, "-o", &dll, "-Wall", "-g", "-shared", "-fPIC"])
      .spawn()
      .expect("error | ToolNotFound: clang");
  Library::new(&dll)
}