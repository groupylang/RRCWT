extern crate rcwt;
extern crate rlisp;
use rlisp::Ast;
use rlisp::error::show_trace;
use rlisp::interpreter::Interpreter;
use rlisp::compiler::IrCompiler;
use rcwt::vm::VirtualMachine;

use std::io;

fn prompt(s: &str) -> io::Result<()> {
  use std::io::{stdout, Write};
  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write(s.as_bytes())?;
  stdout.flush()
}

fn main() {
  use std::io::{stdin, BufRead, BufReader};
  let mut interp = Interpreter::new();
  let mut compiler = IrCompiler::new();

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
      let ir = compiler.compile(&ast);
      println!("{}", ir);
      use std::io::{Write, BufWriter};
      use std::fs::File;
      let mut writer = BufWriter::new(File::create("../tmp/tmp.ir").unwrap());
      writer.write(ir.as_bytes()).unwrap();
    } else {
      break;
    }
  }
  let mut vm = VirtualMachine::scan("tmp/main");
  vm.execute();
}
