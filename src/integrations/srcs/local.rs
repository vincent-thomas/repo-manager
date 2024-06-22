use super::{list::Listable, ListItem};
use crate::config::Configuration;

#[derive(Debug, Default)]
pub struct LocalSource;

impl Listable for LocalSource {
  fn list_repos(&self, config: &Configuration) -> Result<Vec<ListItem>, ()> {
    let dir = std::fs::read_dir(&config.project_directory).unwrap();

    Ok(
      dir
        .into_iter()
        .map(|value| {
          let new_value = value.unwrap();
          let path_buffer = new_value.path();

          let path_name: String = path_buffer.into_os_string().into_string().unwrap();

          let project_name = path_name.split('/').last().unwrap();

          ListItem {
            icon: None,
            name: project_name.to_string(),
            ltype: super::ListType::Local {
              path: new_value.path(),
            },
          }
        })
        .collect(),
    )
  }
}
