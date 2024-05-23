use crate::config;
use crate::config::Configuration;
use crate::integrations::editor;
use crate::integrations::search;
use crate::integrations::srcs::Repository;
use crate::integrations::srcs::{gh::GhSource, local::LocalSource, Source};
use crate::utils::make_sources_unique;

use clier::display::label::LabelLogger;
use clier::display::Displayer;
use clier::hooks::use_double_dash;
use clier::{run::ExitCode, Clier, HasMeta, Runnable};
use std::path::Path;

fn get_sources<T>(_dir: T) -> Vec<Repository>
where
  T: AsRef<Path>,
{
  let total_vec = [GhSource::list(), LocalSource::list()]
    .iter()
    .flatten()
    .cloned()
    .collect::<Vec<Repository>>();
  make_sources_unique(&total_vec)
}

fn get_repo(config: &Configuration, initial_query: &str) -> Option<Repository> {
  let log = LabelLogger::default();
  let repos = get_sources(config.project_directory.clone());

  let searcher = match search::get_searcher(config) {
    Some(value) => value,
    None => {
      log.error("No searcher configured");
      std::process::exit(1);
    }
  };

  if let Some(value) = repos.iter().find(|value| value.repo_id == initial_query) {
    return Some(value.clone());
  }

  if let Ok(value) = searcher.search(repos, initial_query) {
    Some(value)
  } else {
    None
  }
}

pub fn goto_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let initial_query = use_double_dash(&clier).unwrap_or("".into());

  let config = config::load_config();
  let log = LabelLogger::default();

  let repo_selected = get_repo(&config, &initial_query);
  let gotten_editor = match editor::get_editor(&config) {
    Some(editor) => editor,
    None => {
      log.error("No editor configured");
      std::process::exit(1);
    }
  };

  match repo_selected {
    Some(repo) => {
      let _ = gotten_editor.open(&repo.link.resolve_path());
      ExitCode(0)
    }
    None => ExitCode(1),
  }
}
