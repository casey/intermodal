fn main() {
  if let Err(code) = imdl::run() {
    std::process::exit(code);
  }
}
