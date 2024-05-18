use clier::display::Displayer;
use serde::{Deserialize, Serialize};

use crate::utils::UnwrapAnd;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
  pub project_directory: String,
}

pub fn load_config() -> Configuration {
  let log_err = Displayer::Error {};
  let home_config_dir = std::env::var("XDG_CONFIG_HOME");

  let default_dir = format!("{}/.config", std::env::var("HOME").unwrap());

  let contents = std::fs::read_to_string(format!(
    "{}/gitm/config.json",
    home_config_dir.unwrap_or(default_dir)
  ))
  .expect("Config doesn't exist");

  let config: Configuration = serde_json::from_str(contents.as_str()).unwrap_and(|_| {
    log_err.write_err("Config file is required");
    1
  });

  config
}
