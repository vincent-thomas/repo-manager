use std::io;

use clier::display::{label::LabelLogger, Displayer};
use serde::{Deserialize, Serialize};

use crate::utils::UnwrapAnd;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Editors {
  VSCode,
  Neovim,
}

#[derive(Deserialize, Debug, Copy, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Searchers {
  Fzf,
  Pick,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Configuration {
  pub project_directory: String,
  pub editor: Option<Editors>,
  pub searcher: Option<Searchers>,
}

pub fn write_config(config: Configuration) -> io::Result<()> {
  let default_dir = format!(
    "{}/.config/gitm",
    std::env::var("HOME")
      .expect("Neither Variable $HOME nor $XDG_CONFIG_HOME is not defined, this is required for recognizing config directory")
  );

  let string = serde_json::to_string(&config).unwrap();

  let path = format!("{}/config.json", default_dir);

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
    "{}/gitm/config.json",
    home_config_dir.unwrap_or(default_dir)
  ))
  .expect("Config doesn't exist");

  let config: Configuration = serde_json::from_str(contents.as_str()).unwrap_and(|_| {
    log.error("Config file invalid");
    1
  });

  config
}
