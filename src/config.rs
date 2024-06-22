use std::io;

use clier::display::{label::LabelLogger, Displayer};
use serde::{Deserialize, Serialize};

use crate::utils::UnwrapAnd;

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Editors {
  VSCode,
  #[default]
  Neovim,
}

#[derive(Deserialize, Debug, Copy, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Searchers {
  #[default]
  Fzf,
  Pick,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Configuration {
  pub project_directory: String,
  pub editor: Editors,
  pub searcher: Searchers,
}

impl Default for Configuration {
  fn default() -> Self {
    let project_directory = format!(
      "{}/Projects",
      std::env::var("HOME").expect("$HOME not defined")
    );

    Self {
      searcher: Searchers::default(),
      editor: Editors::default(),
      project_directory,
    }
  }
}

pub fn write_config(config: Configuration) -> io::Result<()> {
  let default_dir = format!(
    "{}/.config/gitm",
    std::env::var("HOME")
      .expect("Neither Variable $HOME nor $XDG_CONFIG_HOME is not defined, this is required for recognizing config directory")
  );

  let string = serde_yaml::to_string(&config).unwrap();

  let path = format!("{}/config.yml", default_dir);

  std::fs::write(path, string)
}

pub fn load_config() -> Configuration {
  let log = LabelLogger::default();
  let home_config_dir = std::env::var("XDG_CONFIG_HOME");

  let default_dir = format!(
    "{}/.config",
    std::env::var("HOME")
      .expect("Neither Variable $HOME nor $XDG_CONFIG_HOME is not defined, this is required for recognizing config directory")
  );

  let contents = std::fs::read_to_string(format!(
    "{}/gitm/config.yml",
    home_config_dir.unwrap_or(default_dir)
  ))
  .expect("Config doesn't exist");

  let config: Configuration = serde_yaml::from_str(contents.as_str()).unwrap_and(|_| {
    log.error("Config file invalid");
    1
  });

  config
}
