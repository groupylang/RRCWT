extern crate rcwt;

use rcwt::vm::VirtualMachine;

fn main() {
  let mut vm = VirtualMachine::scan("tmp/main");
  vm.execute();
  vm.jit("a", 0, 4);
}
