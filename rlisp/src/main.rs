extern crate rlisp;
use rlisp::Ast;
use rlisp::error::show_trace;
use rlisp::interpreter::Interpreter;
use rlisp::compiler::IrCompiler;

use std::io;

fn help() {
  println!("select a file to compile or 1 option from the ones below:");
  println!("  \"-h\": show help");
  println!("  \"-v\": show version");
  println!("  \"-i\": use interpreter");
}

fn version() {
  println!("rlisp");
  println!("  a toy lisp processor");
  println!("");
  println!("  version = \"0.1.0\"");
  println!("  authors = [\"sKyrBBit <iamskyrabbit@gmail.com>\"]");
  println!("  license = \"MIT\"");
}

fn prompt(s: &str) -> io::Result<()> {
  use std::io::{stdout, Write};
  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write(s.as_bytes())?;
  stdout.flush()
}

fn interp() {
  use std::io::{stdin, BufRead, BufReader};
  let mut interp = Interpreter::new();

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    prompt("> ").unwrap();
    if let Some(Ok(line)) = lines.next() {
      let ast = match line.parse::<Ast>() {
        Ok(ast) => ast,
        Err(e) => {
          e.show_diagnostic(&line);
          show_trace(e);
          continue;
        }
      };
      println!("{:?}", ast);
      let n = match interp.eval(&ast) {
        Ok(n) => n,
        Err(e) => {
          e.show_diagnostic(&line);
          show_trace(e);
          continue;
        }
      };
      println!("{}", n);
    } else {
      break;
    }
  }
}

fn compile(file_name: &str) {
  use std::io::{BufRead, BufReader};
  use std::fs::File;
  let mut compiler = IrCompiler::new();

  let reader = BufReader::new(File::open(format!("{}.lisp", file_name)).expect("error | FileNotFound"));
  let mut lines = reader.lines();
  if let Some(Ok(line)) = lines.next() {
    let ast = match line.parse::<Ast>() {
      Ok(ast) => ast,
      Err(e) => {
        e.show_diagnostic(&line);
        show_trace(e);
        return;
      }
    };
    println!("{:?}", ast);
    let ir = compiler.compile(&ast);
    println!("{:?}", ir);
    use std::io::{Write, BufWriter};
    let mut writer = BufWriter::new(File::create(format!("{}.wc", file_name)).unwrap());
    writer.write("RCWT".as_bytes()).unwrap();
    writer.write(ir.gen().as_slice()).unwrap();
  }
}

fn main() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 if args[1].eq(&String::from("-h")) => help(),
    2 if args[1].eq(&String::from("-v")) => version(),
    2 if args[1].eq(&String::from("-i")) => interp(),
    2 => compile(&args[1]),
    _ => help()
  };
}