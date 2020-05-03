extern crate cc;
#[cfg(target_os = "linux")]
fn main() {
  cc::Build::new()
  	.cpp(true)
  	.warnings(true)
	  .flag("-Wno-format-security")
	  .flag("-pthread")
  	.file("src/c/env.cpp")
    .file("src/c/vm.cpp")
		.file("src/c/debug.cpp")
		.file("src/c/jit.cpp")
  	.include("src/c")
	.compile("core")  
}
#[cfg(target_os = "windows")]
fn main() {
  	cc::Build::new()
  	.cpp(true)
  	.warnings(true)
  	.file("src/c/env.cpp")
    .file("src/c/vm.cpp")
		.file("src/c/debug.cpp")
		.file("src/c/jit.cpp")
  	.include("src/c")
  	.compile("core")
}