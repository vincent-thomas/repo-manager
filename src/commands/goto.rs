use std::any::Any;
use std::path::PathBuf;
use std::rc::Rc;

use crate::config;
use crate::config::Configuration;
use crate::integrations::editor;
use crate::integrations::search;
use crate::integrations::srcs::list::Listable;
use crate::integrations::srcs::local::LocalSource;
use crate::integrations::srcs::protocols::git::GitProtocol;
use crate::integrations::srcs::GithubSource;
// use crate::integrations::srcs::GitlabSource;
use crate::integrations::srcs::ListItem;
use crate::utils::make_sources_unique;

use clier::display::label::LabelLogger;
use clier::display::Displayer;
use clier::hooks::use_double_dash;
use clier::{run::ExitCode, Clier, HasMeta, Runnable};

fn get_sources(config: &Configuration) -> Vec<ListItem> {
  let github = GithubSource::default();
  let local = LocalSource::default();
  // let gitlab = GitlabSource::default();

  let ghsource = github.list_repos(config).unwrap();
  let localsource = local.list_repos(config).unwrap();
  // let glsource = gitlab.list_repos(config).unwrap();

  let mut total_vec: Vec<ListItem> = Vec::new();

  total_vec.extend(ghsource);
  total_vec.extend(localsource);
  // total_vec.extend(glsource);
  make_sources_unique(&total_vec)
}

fn get_repo(config: &Configuration, initial_query: &str) -> Option<ListItem> {
  let log = LabelLogger::default();
  let repos = get_sources(config);

  if let Some(value) = repos.iter().find(|value| value.name == initial_query) {
    return Some(value.clone());
  }

  let searcher = match search::get_searcher(config) {
    Some(value) => value,
    None => {
      log.error("No searcher configured");
      std::process::exit(1);
    }
  };

  if let Ok(value) = searcher.search(&repos, initial_query).cloned() {
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
  dbg!("test");
  let gotten_editor = match editor::get_editor(&config) {
    Some(editor) => editor,
    None => {
      log.error("No editor configured");
      std::process::exit(1);
    }
  };

  match repo_selected {
    Some(repo) => {
      let mut path = PathBuf::new();

      path.push(config.project_directory);
      path.push(&repo.name);

      dbg!(&path);

      let _ = gotten_editor.open(repo.resolve_path::<GithubSource>(Rc::new(GitProtocol), path));
      ExitCode(0)
    }
    None => ExitCode(1),
  }
}
