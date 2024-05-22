use std::collections::HashSet;

use crate::integrations::srcs::{Link, Repository};

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

pub fn make_sources_unique(vec: &[Repository]) -> Vec<Repository> {
  let mut seen = HashSet::new();
  let mut result = Vec::new();

  for item in vec {
    if let Link::Path(ref _unused) = item.link {
      if seen.insert(&item.repo_id) {
        result.push(item.clone());
      }
    }
  }

  for item in vec {
    if let Link::Url(ref _unused) = item.link {
      if seen.insert(&item.repo_id) {
        result.push(item.clone());
      }
    }
  }

  result
}
