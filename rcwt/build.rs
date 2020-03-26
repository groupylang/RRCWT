extern crate cc;
#[cfg(target_os = "linux")]
fn main() {
  cc::Build::new()
  .cpp(true)
  .warnings(true)
  .flag("-Wall")
  .flag("-Wextra")
  .flag("-Wno-format-security")
  .file("src/c/env.cpp")
  .file("src/c/vm.cpp")
  .include("src/c")
  .compile("core")
}
#[cfg(target_os = "windows")]
fn main() {
  cc::Build::new()
  .cpp(true)
  .warnings(true)
  .flag("/Wall")
  .file("src/c/env.cpp")
  .file("src/c/vm.cpp")
  .include("src/c")
  .compile("core")
}