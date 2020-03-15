extern crate cc;
#[cfg(target_os = "linux")]
fn main() {
  cc::Build::new()
  .warnings(true)
  .flag("-Wall")
  .flag("-Wextra")
  .file("src/c/core.c")
  .file("src/c/native.c")
  .include("src/c")
  .compile("core")
}
#[cfg(target_os = "windows")]
fn main() {
  cc::Build::new()
  .warnings(true)
  .flag("/Wall")
  .file("src/c/core.c")
  .file("src/c/native.c")
  .include("src/c")
  .compile("core")
}