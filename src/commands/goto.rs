use crate::config;
use crate::integrations::editor;
use crate::integrations::search;

use clier::display::label::LabelLogger;
use clier::display::Displayer;
use clier::{hooks::use_flag, run::ExitCode, Clier, HasMeta, Runnable};
use std::path::Path;

pub fn prepare_list_of_dir<T>(dir: T) -> Vec<String>
where
  T: AsRef<Path>,
{
  let dir = std::fs::read_dir(dir).unwrap();

  dir
    .into_iter()
    .map(|value| {
      value
        .unwrap()
        .path()
        .into_os_string()
        .into_string()
        .unwrap()
    })
    .collect()
}

pub fn togo_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let config = config::load_config();
  let log = LabelLogger::default();

  let where_to_flag = use_flag("where", Some('w'), &clier)
    .try_into()
    .ok()
    .map(|dir: String| format!("{}/{dir}", config.project_directory));

  let where_to = match where_to_flag {
    Some(value) => value,
    None => {
      let dir = prepare_list_of_dir(&config.project_directory);
      let searcher = match search::get_searcher(&config) {
        Some(value) => value,
        None => {
          log.error("No searcher configured");
          std::process::exit(1);
        }
      };

      if let Ok(value) = searcher.search(dir) {
        value
      } else {
        return ExitCode(0);
      }
    }
  };
  let gotten_editor = match editor::get_editor(&config) {
    Some(editor) => editor,
    None => {
      log.error("No editor configured");
      std::process::exit(1);
    }
  };

  let _result = gotten_editor.open(&where_to);
  ExitCode(0)
}
