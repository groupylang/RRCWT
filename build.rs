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