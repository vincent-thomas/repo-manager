use super::{Repository, Source};
use crate::config;

#[derive(Debug)]
pub struct LocalSource;

impl Source for LocalSource {
  fn list() -> Vec<Repository> {
    let config = config::load_config();

    let dir = std::fs::read_dir(config.project_directory).unwrap();

    dir
      .into_iter()
      .map(|value| {
        let value_buf = value.unwrap().path();

        let test = value_buf.into_os_string().into_string().unwrap();

        let project_name = test.split('/').last().unwrap();

        Repository {
          repo_id: project_name.to_string(),
          display_name: project_name.to_string(),
          link: super::Link::Path(test),
        }
      })
      .collect()
  }
}
