fn main() {
  if let Err(code) = imdl::run(std::env::args()) {
    std::process::exit(code);
  }
}
