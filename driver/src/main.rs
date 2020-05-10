//! command-line driver for RRCWT
extern crate serde;
extern crate clap;
extern crate rcwt;

use serde::{Deserialize, Serialize};
use clap::{App, Arg, SubCommand};

use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter, Write};
use std::process::Command;
use std::ffi::CString;
use std::time::Instant;
use std::{io, fs};

use rcwt::vm::VirtualMachine;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
  builder: Builder,
  runner: Runner,
}
#[derive(Serialize, Deserialize, Debug)]
struct Builder {
  path: String,
  debug: bool
}
#[derive(Serialize, Deserialize, Debug)]
struct Runner {
  path: String,
  jit: bool
}

struct ConfigBuilder {
  builder_path: String,
  debug: bool,
  runner_path: String,
  jit: bool
}

impl ConfigBuilder {
  fn new(path: &str) -> Self {
    ConfigBuilder {
      builder_path: String::from(path),
      debug: false,
      runner_path: format!("tmp/{}", path),
      jit: false
    }
  }
  fn builder_path(&mut self, opt_path: Option<&str>) -> &mut Self {
    if let Some(path) = opt_path {
      self.builder_path = String::from(path);
    }
    self
  }
  fn debug(&mut self, debug: bool) -> &mut Self {
    self.debug = debug;
    self
  }
  fn runner_path(&mut self, opt_path: Option<&str>) -> &mut Self {
    if let Some(path) = opt_path {
      self.runner_path = String::from(path);
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
        path: std::mem::take(&mut self.builder_path),
        debug: self.debug
      },
      runner: Runner {
        path: std::mem::take(&mut self.runner_path),
        jit: self.jit
      }
    }
  }
}

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

fn write_config(path: &str, config: Config) -> io::Result<()> {
  let path = std::env::current_dir()?.join(path);
  let workspace = path.parent().unwrap();
  let writer = BufWriter::new(File::create(workspace.join("config.yml")).expect("error | ConfigNotFound"));
  serde_yaml::to_writer(writer, &config).expect("error | InvalidYamlFile");
  Ok(())
}

fn read_config(path: &str) -> io::Result<Config> {
  let reader = BufReader::new(
    File::open(std::env::current_dir()?.join(path).join("config.yml")
  ).expect("error | ConfigNotFound"));
  let config: Config = serde_yaml::from_reader(reader).expect("error | InvalidYamlFile");
  Ok(config)
}

fn main() -> io::Result<()> {
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
    )
    .subcommand(SubCommand::with_name("direct")
      .about("Invoce RCWT virtual machine directly")
      .arg(Arg::with_name("path")
        .help("Path of yaml file")
        .takes_value(true)
        .required(true)
      )
    );

  let matches = driver.get_matches();
  // new
  if let Some(ref matches) = matches.subcommand_matches("new") {
    // create workspace
    let path = matches.value_of("path").unwrap();
    let workspace = std::env::current_dir()?.join(path);
    create_dir_all(&workspace)?;
    // create main.grp
    let main = workspace.join("main.grp");
    let mut writer = BufWriter::new(File::create(&main)?);
    writer.write_all("Integer main() {\n\tprint \"Hello, World!\";\n\treturn 0;\n}".as_bytes())?;
    // create config.yml
    let config = ConfigBuilder::new(&format!("{}/main", path))
      .builder_path(matches.value_of("builder-input"))
      .debug(matches.is_present("debug"))
      .runner_path(matches.value_of("runner-input"))
      .jit(matches.is_present("jit"))
      .build();
    write_config(main.to_str().unwrap(), config)?;
  }
  // build
  if let Some(ref matches) = matches.subcommand_matches("build") {
    let workspace = matches.value_of("workspace").unwrap();
    let config = read_config(workspace)?;
    if config.builder.debug {
      Command::new("java")
        .args(&["-classpath", "javaout", "driver/Driver", &config.builder.path, "-d"])
        .spawn().unwrap();
    } else {
      Command::new("java")
        .args(&["-classpath", "javaout", "driver/Driver", &config.builder.path])
        .spawn().unwrap();
    }
  }
  // run
  if let Some(ref matches) = matches.subcommand_matches("run") {
    let workspace = matches.value_of("workspace").unwrap();
    let config = read_config(workspace)?;
    if config.runner.jit {
      match VirtualMachine::scan(&config.runner.path) {
        Ok(mut vm) => vm.execute(),
        Err(msg) => eprintln!("{:#?}", msg)
      }
    } else {
      match VirtualMachine::scan(&config.runner.path) {
        Ok(mut vm) => vm.execute(),
        Err(msg) => eprintln!("{:#?}", msg)
      }
    }
  }
  // direct
  if let Some(ref matches) = matches.subcommand_matches("direct") {
    let path = matches.value_of("path").unwrap();
    fs::create_dir_all("tmp")?;
    let reader = BufReader::new(File::open(format!("{}.yml", path))?);
    let direct: Direct = serde_yaml::from_reader(reader).expect("error | InvalidYamlFile");

    #[link(name="core")]
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
  }
  Ok(())
}