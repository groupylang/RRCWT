//! command-line driver for RRCWT
extern crate serde;
extern crate clap;
extern crate rcwt;

use serde::{Deserialize, Serialize};
use clap::{App, Arg, SubCommand};

use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter, Write};
use std::process::Command;

use rcwt::vm::VirtualMachine;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
  builder: Builder,
  runner: Runner,
}
#[derive(Serialize, Deserialize, Debug)]
struct Builder {
  input: String,
  debug: bool
}
#[derive(Serialize, Deserialize, Debug)]
struct Runner {
  input: String,
  jit: bool
}

struct ConfigBuilder {
  builder_input: String,
  debug: bool,
  runner_input: String,
  jit: bool
}

impl ConfigBuilder {
  fn new(input: &str) -> Self {
    ConfigBuilder {
      builder_input: String::from(input),
      debug: false,
      runner_input: format!("tmp/{}", input),
      jit: false
    }
  }
  fn builder_input(&mut self, opt_input: Option<&str>) -> &mut Self {
    if let Some(input) = opt_input {
      self.builder_input = String::from(input);
    }
    self
  }
  fn debug(&mut self, debug: bool) -> &mut Self {
    self.debug = debug;
    self
  }
  fn runner_input(&mut self, opt_input: Option<&str>) -> &mut Self {
    if let Some(input) = opt_input {
      self.runner_input = String::from(input);
    }
    self
  }
  fn jit(&mut self, jit: bool) -> &mut Self {
    self.jit = jit;
    self
  }
  fn build(&mut self) -> Config {
    Config {
      builder: Builder {
        input: std::mem::take(&mut self.builder_input),
        debug: self.debug
      },
      runner: Runner {
        input: std::mem::take(&mut self.runner_input),
        jit: self.jit
      }
    }
  }
}

fn write_config(file_name: &str, config: Config) {
  let input = std::env::current_dir().unwrap().join(file_name);
  let workspace = input.parent().unwrap();
  let writer = BufWriter::new(File::create(workspace.join("config.yml")).expect("error | ConfigNotFound"));
  serde_yaml::to_writer(writer, &config).expect("error | InvalidYamlFile");
}

fn read_config(path: &str) -> Config {
  let reader = BufReader::new(
    File::open(std::env::current_dir().unwrap().join(path).join("config.yml")
  ).expect("error | ConfigNotFound"));
  let config: Config = serde_yaml::from_reader(reader).expect("error | InvalidYamlFile");
  config
}

fn main() {
  let driver = App::new("rrcwt")
    .version("0.1.0")
    .author("skyrabbit <iamskyrabbit@gmail.com>")
    .about("a toy processor")
    .subcommand(SubCommand::with_name("new")
      .about("Creates new workspace")
      .arg(Arg::with_name("path")
        .help("Path of workspace")
        .takes_value(true)
        .required(true)
      )
      .arg(Arg::with_name("builder-input")
        .help("Path of builder input")
        .short("b")
        .long("builder-input")
        .takes_value(true)
      )
      .arg(Arg::with_name("debug")
        .help("Enables debug")
        .short("d")
        .long("debug")
      )
      .arg(Arg::with_name("runner-input")
        .help("Path of runner input")
        .short("r")
        .long("runner-input")
        .takes_value(true)
      )
      .arg(Arg::with_name("jit")
        .help("Enables jit")
        .short("j")
        .long("jit")
      )
    )
    .subcommand(SubCommand::with_name("build")
      .about("Builds with compiler")
      .arg(Arg::with_name("workspace")
        .help("Path of workspace")
        .takes_value(true)
        .required(true)
      )
    )
    .subcommand(SubCommand::with_name("run")
      .about("Runs with virtual machine")
      .arg(Arg::with_name("workspace")
        .help("Path of workspace")
        .takes_value(true)
        .required(true)
      )
    );

  let matches = driver.get_matches();
  // new
  if let Some(ref matches) = matches.subcommand_matches("new") {
    // create workspace
    let path = matches.value_of("path").unwrap();
    let workspace = std::env::current_dir().unwrap().join(path);
    create_dir_all(&workspace).unwrap();
    // create main.grp
    let main = workspace.join("main.grp");
    let mut writer = BufWriter::new(File::create(&main).unwrap());
    writer.write_all("Integer main() {\n\tprint \"Hello, World!\";\n\treturn 0;\n}".as_bytes()).unwrap();
    // create config.yml
    let config = ConfigBuilder::new(&format!("{}/main", path))
      .builder_input(matches.value_of("builder-input"))
      .debug(matches.is_present("debug"))
      .runner_input(matches.value_of("runner-input"))
      .jit(matches.is_present("jit"))
      .build();
    write_config(main.to_str().unwrap(), config);
  }
  if let Some(ref matches) = matches.subcommand_matches("build") {
    let workspace = matches.value_of("workspace").unwrap();
    let config = read_config(workspace);
    if config.builder.debug {
      Command::new("java")
        .args(&["-classpath", "javaout", "driver/Driver", &config.builder.input, "-d"])
        .spawn().unwrap();
    } else {
      Command::new("java")
        .args(&["-classpath", "javaout", "driver/Driver", &config.builder.input])
        .spawn().unwrap();
    }
  }
  if let Some(ref matches) = matches.subcommand_matches("run") {
    let workspace = matches.value_of("workspace").unwrap();
    let config = read_config(workspace);
    if config.runner.jit {
      match VirtualMachine::scan(&config.runner.input) {
        Ok(mut vm) => vm.execute(),
        Err(msg) => eprintln!("{:#?}", msg)
      }
    } else {
      match VirtualMachine::scan(&config.runner.input) {
        Ok(mut vm) => vm.execute(),
        Err(msg) => eprintln!("{:#?}", msg)
      }
    }
  }
}