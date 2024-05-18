pub trait UnwrapAnd<T, E> {
  fn unwrap_and<F: Fn(E) -> i32>(self, err_fn: F) -> T;
}

impl<T, E> UnwrapAnd<T, E> for Result<T, E> {
  fn unwrap_and<F: Fn(E) -> i32>(self, err_fn: F) -> T {
    match self {
      Ok(value) => value,
      Err(err) => {
        let exit_code = err_fn(err);
        std::process::exit(exit_code);
      }
    }
  }
}
