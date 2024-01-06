use crate::common::*;

/// Entry point into the IMDL binary.
///
/// Called by `main` in `main.rs`.
///
/// Constructs an `Env` using `Env::main`, containing the command line
/// arguments, the current working directory, a handle to standard error,
/// standard output, and standard input.
///
/// Calls `Env::status` which runs the program, prints error messages, and
/// returns a status code in case there was an error.
///
/// Errors
/// ------
///
/// In case of an error, a nonzero status code is returned. This status code can
/// be passed to `std::process::exit`, to exit the process and report its
/// failure to the system.
pub fn run() -> Result<(), i32> {
  let mut env = match Env::main() {
    Ok(env) => env,
    Err(err) => {
      eprintln!("{err}");
      return Err(EXIT_FAILURE);
    }
  };

  env.status()
}
