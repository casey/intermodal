macro_rules! matches {
  ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
    match $expression {
      $( $pattern )|+ $( if $guard )? => true,
      _ => false
    }
  }
}
