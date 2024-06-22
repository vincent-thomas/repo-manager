use crate::config::Configuration;

use super::{ListItem, Source};

enum Repository<T: Source> {
  Remote { repo_id: String, source: T },
  Local { path: String, repo_id: String },
}

pub trait Listable {
  fn list_repos(&self, config: &Configuration) -> Result<Vec<ListItem>, ()>;
}
