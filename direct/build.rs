extern crate cc;
#[cfg(target_os = "linux")]
fn main() {
  cc::Build::new()
  	.cpp(true)
  	.warnings(true)
	  .flag("-Wno-format-security")
	  .flag("-pthread")
  	.file("../rcwt/src/c/env.cpp")
    .file("../rcwt/src/c/vm.cpp")
		.file("../rcwt/src/c/debug.cpp")
		.file("../rcwt/src/c/jit.cpp")
  	.include("../rcwt/src/c")
	.compile("core")  
}
#[cfg(target_os = "windows")]
fn main() {
  	cc::Build::new()
  	.cpp(true)
  	.warnings(true)
  	.file("../rcwt/src/c/env.cpp")
    .file("../rcwt/src/c/vm.cpp")
		.file("../rcwt/src/c/debug.cpp")
		.file("../rcwt/src/c/jit.cpp")
  	.include("../rcwt/src/c")
  	.compile("core")
}