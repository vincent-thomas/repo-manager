use std::fmt;

use crate::config::{Configuration, Searchers};

use super::srcs::{ListItem, Source};

pub mod fzf;
pub mod pick;

pub trait Searcher: fmt::Debug {
  // FIXME: venne
  fn search<'a>(&self, list: &'a [ListItem], initial_search: &str) -> Result<&'a ListItem, ()>;
}

pub fn get_searcher<'a>(config: &Configuration) -> Option<&'a dyn Searcher> {
  static FZF: fzf::Fzf = fzf::Fzf;
  static PICK: pick::Pick = pick::Pick;

  let editor: &dyn Searcher = match config.searcher {
    Searchers::Fzf => &FZF,
    Searchers::Pick => &PICK,
  };

  Some(editor)
}
