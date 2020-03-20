extern crate rcwt;
extern crate rlisp;
use rcwt::vm::VirtualMachine;


fn help() {
  println!("put paths of files to execute or 1 option from the ones below:");
  println!("  \"-h\": show help");
  println!("  \"-v\": show version");
}

fn version() {
  println!("rcwt");
  println!("  a toy virtual machine");
  println!("");
  println!("  version = \"0.1.0\"");
  println!("  authors = [\"sKyrBBit <iamskyrabbit@gmail.com>\"]");
  println!("  license = \"MIT\"");
}

fn main() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => help(),
    2 if args[1].eq(&String::from("-h")) => help(),
    2 if args[1].eq(&String::from("-v")) => version(),
    _ => VirtualMachine::scan(&args[1]).execute(),
  };
}
