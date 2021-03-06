extern crate rcwt;
use rcwt::vm::VirtualMachine;

fn help() {
  println!("select a file to execute or 1 option from the ones below:");
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
    2 if args[1].eq(&String::from("-h")) => help(),
    2 if args[1].eq(&String::from("-v")) => version(),
    2 => {
      match VirtualMachine::scan(&args[1]) {
        Ok(mut vm) => vm.execute(),
        Err(msg) => eprintln!("{:#?}", msg)
      }
    },
    _ => help(),
  };
}
