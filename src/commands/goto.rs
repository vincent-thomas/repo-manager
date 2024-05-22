use crate::config;
use crate::integrations::editor;
use crate::integrations::search;
use crate::integrations::srcs::Repository;
use crate::integrations::srcs::{gh::GhSource, git::GitSource, Source};
use crate::utils::make_sources_unique;

use clier::display::label::LabelLogger;
use clier::display::Displayer;
use clier::{run::ExitCode, Clier, HasMeta, Runnable};
use std::path::Path;

fn get_list<T>(_dir: T) -> Vec<Repository>
where
  T: AsRef<Path>,
{
  let total_vec = [GhSource::list(), GitSource::list()]
    .iter()
    .flatten()
    .cloned()
    .collect::<Vec<Repository>>();
  let vec: Vec<Repository> = make_sources_unique(&total_vec);
  vec
}

pub fn togo_command(_clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let config = config::load_config();
  let log = LabelLogger::default();

  let where_to = {
    let dir = get_list(&config.project_directory);
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
  };
  let gotten_editor = match editor::get_editor(&config) {
    Some(editor) => editor,
    None => {
      log.error("No editor configured");
      std::process::exit(1);
    }
  };

  let link = where_to.link;

  let _result = gotten_editor.open(&link.resolve_path());
  ExitCode(0)
}
