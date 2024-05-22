use std::process::{Command, Stdio};

use crate::config;

pub mod gh;
pub mod git;

#[derive(Debug, Clone)]
pub enum Link {
  Url(String),
  Path(String),
}

#[derive(Debug, Clone)]
pub struct Repository {
  /// This can be url or
  pub repo_id: String,
  pub display_name: String,
  pub link: Link,
}

impl Link {
  pub fn resolve_path(self) -> String {
    let config = config::load_config();

    match self {
      Self::Url(url) => {
        let url_name = url.split('/').last();
        let path = format!(
          "{}/{}",
          &config.project_directory,
          // FIXME: unwrap
          url_name.unwrap_or_else(|| panic!("Invalid git url: {:?}", url_name))
        );
        let _ = Command::new("git")
          .args(["clone", &url, &path])
          .stdin(Stdio::piped())
          .stderr(Stdio::piped())
          .output()
          .expect("'git' command failed, maybe it's not available in path?");

        path
      }
      Self::Path(path) => path,
    }
  }
}

pub trait Source {
  fn list() -> Vec<Repository>;
}
