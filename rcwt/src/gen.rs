use std::fs::File;
use std::io::{Write, BufWriter};

pub fn gen_hir() {
  let mut writer = BufWriter::new(File::create("tmp/test.hir").unwrap());
  writer.write("<text>\n".as_bytes()).unwrap();
  writer.write("main: []\n".as_bytes()).unwrap();
  writer.write("\tconst R1 s0; # addi  R1 R0 0x00\n".as_bytes()).unwrap();
  writer.write("\tconst R2 0x00 # addi  R2 R0 0x00\n".as_bytes()).unwrap();
  writer.write("\tconst R3 0x05 # addi  R3 R0 0x05\n".as_bytes()).unwrap();
  writer.write("L0;\n".as_bytes()).unwrap();
  writer.write("\tifgt  L1; R2 R3 # ifgt  0x05 R2 R3\n".as_bytes()).unwrap();
  writer.write("\tpush  R1\n".as_bytes()).unwrap();
  writer.write("\tcall  sub: # call  0x00 0x09\n".as_bytes()).unwrap();
  writer.write("\taddi  R2 R2 0x01\n".as_bytes()).unwrap();
  writer.write("\tgoto  L0; # goto  0xfc\n".as_bytes()).unwrap();
  writer.write("L1;\n".as_bytes()).unwrap();
  writer.write("\texit\n".as_bytes()).unwrap();
  writer.write("sub: []\n".as_bytes()).unwrap();
  writer.write("\tload  R4 0xff\n".as_bytes()).unwrap();
  writer.write("\tsout  R4\n".as_bytes()).unwrap();
  writer.write("\tret\n".as_bytes()).unwrap();
  writer.write("<data>\n".as_bytes()).unwrap();
  writer.write("s0;\n".as_bytes()).unwrap();
  writer.write("\t\"Loop\\0\"\n".as_bytes()).unwrap();
}

pub fn gen_lir() {
  let mut writer = BufWriter::new(File::create("tmp/test.lir").unwrap());
  writer.write("addi  01 00 00\n".as_bytes()).unwrap();
  writer.write("addi  02 00 00\n".as_bytes()).unwrap();
  writer.write("addi  03 00 05\n".as_bytes()).unwrap();
  writer.write("ifgt  05 02 03\n".as_bytes()).unwrap();
  writer.write("push  01 00 00\n".as_bytes()).unwrap();
  writer.write("call  00 00 09\n".as_bytes()).unwrap();
  writer.write("addi  02 02 01\n".as_bytes()).unwrap();
  writer.write("goto  fc 00 00\n".as_bytes()).unwrap();
  writer.write("exit  00 00 00\n".as_bytes()).unwrap();
  writer.write("load  04 ff 00\n".as_bytes()).unwrap();
  writer.write("sout  04 00 00\n".as_bytes()).unwrap();
  writer.write("ret   00 00 00\n".as_bytes()).unwrap();
  writer.write("\"Loop\\0\"\n".as_bytes()).unwrap();
}

pub fn gen_wc() {
  // let mut writer = BufWriter::new(File::create("tmp/test.wc").unwrap());
  // writer.write("RCWT".as_bytes()).unwrap();
  // writer.write("\0\0".as_bytes()).unwrap(); // text_size
  // writer.write("\0\0".as_bytes()).unwrap(); // data_size
  // writer.write("\0\0".as_bytes()).unwrap(); // defined_count
  // writer.write("\0\0".as_bytes()).unwrap(); // undefined_count
  // writer.write("\0\0".as_bytes()).unwrap(); // relocation_count
  // // defined_table: [DefinedHeader; defined_count],
  // // undefined_table: [UndefinedHeader; undefined_count],
  // // relocations: [RelocationHeader; relocation_count],
  // writer.write("20 01 00 00".as_bytes()).unwrap();
  // writer.write("\0\0\0\0".as_bytes()).unwrap();
  // writer.write("\0\0\0\0".as_bytes()).unwrap();
  // writer.write("\0\0\0\0".as_bytes()).unwrap();
  // writer.write("\0\0\0\0".as_bytes()).unwrap();
  // writer.write("Loop\0".as_bytes()).unwrap();

  // let entry = BasicBlock { label: String::from("entry"), block: vec![Instruction {
  //   code: 0x20,
  //   kind: InstrKind::RRI { op0: 0x01, op1: 0x00, op2: 0x01 },
  //   mnemonic: String::from("addi")
  // },
  // Instruction {
  //   code: 0x20,
  //   kind: InstrKind::RRI { op0: 0x02, op1: 0x00, op2: 0x02 },
  //   mnemonic: String::from("addi")
  // },
  // Instruction {
  //   code: 0x20,
  //   kind: InstrKind::RRI { op0: 0x03, op1: 0x00, op2: 0x03 },
  //   mnemonic: String::from("addi")
  // },
  // Instruction {
  //   code: 0x12,
  //   kind: InstrKind::RRR { op0: 0x04, op1: 0x02, op2: 0x03 },
  //   mnemonic: String::from("mulr")
  // },
  // Instruction {
  //   code: 0x10,
  //   kind: InstrKind::RRR { op0: 0x05, op1: 0x01, op2: 0x04 },
  //   mnemonic: String::from("addr")
  // },
  // Instruction {
  //   code: 0x41,
  //   kind: InstrKind::SYS,
  //   mnemonic: String::from("exit")
  // }]};
  // println!("{:?}", entry);
  // println!("RCWT");
  // println!("{}", entry);
}