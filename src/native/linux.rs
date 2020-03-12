//! TODO
use super::super::llvm_sys::core::*;
use super::super::llvm_sys::target;
use super::super::llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use super::super::llvm_sys::execution_engine::*;
use super::super::llvm_sys::prelude::LLVMValueRef;
use super::super::llvm_sys::prelude::LLVMBuilderRef;
use super::super::llvm_sys::prelude::LLVMModuleRef;
use std::ffi::CString;
use std::os::raw::{c_char};
use std::rc::Rc;

use super::VirtualMachine;

// one symbols by one module

impl VirtualMachine {
  fn jit(&mut self, base_address: usize, size: usize) {
    let mut assembly = String::from("");
    // compile
    for n in base_address ..  base_address + size {
      let i = self.body.get(n).expect("error | Crash");
      assembly += match i.code {
        0x00 /* NOP   */ => "",
        0x02 /* BP    */ => "",
        0x04 /* STORE */ => "",
        0x05 /* LOAD  */ => "",
        0x06 /* PUSH  */ => "",
        0x07 /* POP   */ => "",
        0x10 /* ADDR  */ => "",
        0x11 /* SUBR  */ => "",
        0x12 /* MULR  */ => "",
        0x13 /* DIVR  */ => "",
        0x14 /* GT    */ => "",
        0x15 /* GE    */ => "",
        0x16 /* EQ    */ => "",
        0x18 /* AND   */ => "",
        0x19 /* OR    */ => "",
        0x1a /* NOT   */ => "",
        0x1c /* SHL   */ => "",
        0x1d /* SHR   */ => "",
        0x20 /* ADDI  */ => "",
        0x21 /* SUBI  */ => "",
        0x22 /* MULI  */ => "",
        0x23 /* DIVI  */ => "",
        0x40 /* GOTO  */ => "",
        0x41 /* EXIT  */ => "",
        0x42 /* CALL  */ => "",
        0x43 /* RET   */ => "",
        0x44 /* IFGT  */ => "",
        0x45 /* IFGE  */ => "",
        0x46 /* IFEQ  */ => "",
        0x50 /* NEW   */ => "",
        0x51 /* SET   */ => "",
        0x52 /* GET   */ => "",
        _ => { println!("warning | InstructionInvalid"); "" }
      };
      // assemble
      // dynamic link
      // 
    }
  }
  fn gen_module() {

  }
  fn gen_function() {

  }
  fn gen_basic_block() {

  }
  fn gen_instruction() {

  }
}
// fn invoke() {
//   let dylib_path = "dll_example1.dll";
//   let symbol_name = "square\0"; // null-terminated

//   let dylib = libloading::Library::new(dylib_path);
//   match dylib {
//     Ok(lib) => {
//       let square_opt: Result<libloading::Symbol<extern fn(i32) -> i32>, _> = unsafe {
//         lib.get(symbol_name.as_bytes())
//       };
//       match square_opt {
//         Ok(square) => {
//           println!("square(2) = {}", square(2))
//         }
//         Err(msg) => {
//           println!("{}: `{}`", msg, symbol_name)
//         }
//       }
//     }
//     Err(msg) => {
//       println!("{}: {}", msg, dylib_path)
//     }
//   }
// }

struct JITCompiler {
  builder: LLVMBuilderRef,
  mod_name: CString,
  module: LLVMModuleRef
}
impl JITCompiler { // TODO
  fn initialize_llvm() {
    unsafe {
      if target::LLVM_InitializeNativeTarget() != 0 {
        panic!("Could not initialize target");
      }
      if target::LLVM_InitializeNativeAsmPrinter() != 0 {
        panic!("Could not initialize ASM Printer");
      }
    }
  }
  fn setup() -> JITCompiler {
    let name1 = CString::new("my_module").unwrap();
    let name2 = CString::new("my_module").unwrap();
    JITCompiler {
      builder: unsafe { LLVMCreateBuilder() },
      mod_name: name1,
      module: unsafe { LLVMModuleCreateWithName(name2.as_ptr()) }
    }
  }
  fn create(&self) {
    let function_type = unsafe {
      let mut param_types = [];
      LLVMFunctionType(LLVMInt32Type(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
    };
    let function_name = CString::new("main").unwrap();
    let function = unsafe { LLVMAddFunction(self.module, function_name.as_ptr(), function_type) };
    let entry_name = CString::new("entry").unwrap();
    let entry_block = unsafe { LLVMAppendBasicBlock(function, entry_name.as_ptr()) };
    unsafe { LLVMPositionBuilderAtEnd(self.builder, entry_block); }
  }
  fn a(&self) -> LLVMValueRef {
    let name = CString::new("a").unwrap();
    let value = unsafe { LLVMBuildAlloca(self.builder, LLVMInt32Type(), name.as_ptr()) };
    unsafe { LLVMBuildStore(self.builder, LLVMConstInt(LLVMInt32Type(), 32, 0), value); }
    value
  }
  fn b(&self) -> LLVMValueRef {
    let name = CString::new("b").unwrap();
    let value = unsafe { LLVMBuildAlloca(self.builder, LLVMInt32Type(), name.as_ptr()) };
    unsafe { LLVMBuildStore(self.builder, LLVMConstInt(LLVMInt32Type(), 16, 0), value); }
    value 
  }
  fn ab(&self, a: LLVMValueRef, b: LLVMValueRef) {
    let b_val_name = CString::new("b_val").unwrap();
    let b_val = unsafe { LLVMBuildLoad(self.builder, b, b_val_name.as_ptr()) };
    let a_val_name = CString::new("a_val").unwrap();
    let a_val = unsafe { LLVMBuildLoad(self.builder, a, a_val_name.as_ptr()) };
    let ab_val_name = CString::new("ab_val").unwrap();
    unsafe {
      let res = LLVMBuildAdd(self.builder, a_val, b_val, ab_val_name.as_ptr());
      LLVMBuildRet(self.builder, res);
    }
  }
  fn varify(&self) {
    let mut error: *mut c_char = 0 as *mut c_char;
    let ok = unsafe {
        let buf: *mut *mut c_char = &mut error;
        LLVMVerifyModule(self.module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
    };
    if ok == 1 {
        let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
        panic!("cannot verify module '{:?}'.\nError: {}", self.mod_name, err_msg);
    }
  }
  fn dump(&self) {
    unsafe { LLVMDumpModule(self.module); }
  }
  fn create_engine(&self) {
    let mut error: *mut c_char = 0 as *mut c_char;
    let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
    let ok = unsafe {
      error = 0 as *mut c_char;
      let buf: *mut *mut c_char = &mut error;
      let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
      LLVMLinkInInterpreter();
      LLVMCreateInterpreterForModule(engine_ref, self.module, buf)
    };
    if ok == 1 {
      let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
      println!("Execution error: {}", err_msg);
    } else {
      // run the function!
      let func_name = CString::new("main").unwrap();
      let named_function = unsafe { LLVMGetNamedFunction(self.module, func_name.as_ptr()) };
      let mut params = [];
      let func_result = unsafe { LLVMRunFunction(engine, named_function, params.len() as u32, params.as_mut_ptr()) };
      let result = unsafe{ LLVMGenericValueToInt(func_result, 0) };
      println!("{} + {} = {}", 32, 16, result);
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {
    super::JITCompiler::initialize_llvm();
    // setup
    let compiler = super::JITCompiler::setup();
    // create
    compiler.create();
    // int a = 32
    let a = compiler.a();
    // int b = 16
    let b = compiler.b();
    // return a + b
    compiler.ab(a, b);
    // verify it's all good
    compiler.varify();
    // Clean up the builder now that we are finished using it.
    // unsafe { LLVMDisposeBuilder(builder) }
    // Dump the LLVM IR to stdout so we can see what we've created
    compiler.dump();
    // create our exe engine
    compiler.create_engine();
    // Clean up the module after we're done with it.
    // unsafe { LLVMDisposeModule(module) }
  }
}