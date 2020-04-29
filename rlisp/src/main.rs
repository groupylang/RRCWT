extern crate rlisp;
use rlisp::ast::Ast;
use rlisp::error::show_trace;
use rlisp::interpreter::Interpreter;
use rlisp::compiler::IrCompiler;

use std::io;
use std::fs;

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
  use io::{stdout, Write};
  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write(s.as_bytes())?;
  stdout.flush()
}

fn interp() -> io::Result<()> {
  use io::{stdin, BufRead, BufReader};
  let mut interp = Interpreter::new();

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    prompt("> ")?;
    if let Some(Ok(line)) = lines.next() {
      if line == "exit" { break; }
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
  };
  Ok(())
}

fn compile(file_name: &str) -> io::Result<()> {
  use io::{BufRead, BufReader};
  use fs::File;
  let mut compiler = IrCompiler::new();

  let reader = BufReader::new(File::open(format!("{}.lisp", file_name))
    .expect(&format!("error | FileNotFound: {}.lisp", file_name)));
  let mut lines = reader.lines();
  if let Some(Ok(line)) = lines.next() {
    let ast = match line.parse::<Ast>() {
      Ok(ast) => ast,
      Err(e) => {
        e.show_diagnostic(&line);
        show_trace(e);
        return Ok(());
      }
    };
    println!("{:?}", ast);
    let ir = compiler.compile(&ast);
    println!("{:?}", ir);
    use io::{Write, BufWriter};
    use fs::create_dir_all;
    use std::path::Path;
    create_dir_all(Path::new(&format!("tmp/{}.wc", file_name)).parent().unwrap())?;
    let mut writer = BufWriter::new(File::create(format!("tmp/{}.wc", file_name))?);
    writer.write("RCWT".as_bytes())?;
    writer.write(ir.gen().as_slice())?;
  };
  Ok(())
}

fn main() -> io::Result<()> {
  use std::env;
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 if args[1].eq("-h") => Ok(help()),
    2 if args[1].eq("-v") => Ok(version()),
    2 if args[1].eq("-i") => interp(),
    2 => compile(&args[1]),
    _ => Ok(help())
  }
}