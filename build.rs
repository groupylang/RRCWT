fn main() {
  cc::Build::new()
  .warnings(true)
  .flag("-Wall")
  .flag("-Wextra")
  .file("src/c/core.c")
  .include("src/c")
  .compile("core")
}