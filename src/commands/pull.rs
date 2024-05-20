use std::{
  io::ErrorKind,
  path::Path,
  process::{Command, Stdio},
};

use clier::{
  display::{label::LabelLogger, Displayer},
  hooks::use_flag,
  run::ExitCode,
  Clier, HasMeta, Runnable,
};

use crate::{config, utils::UnwrapAnd};

pub fn clone_project(repo_url: &str, dir: &str) {
  let log = LabelLogger::default();

  log.info("Cloning...");

  let git_clone_command = Command::new("git")
    .args(["clone", repo_url, dir])
    .stdin(Stdio::piped())
    .stderr(Stdio::piped())
    .output();

  match git_clone_command {
    Ok(value) => {
      if value.status.success() {
        log.success("Pulled project!");
      } else {
        log.error("Error writing project:");
        eprintln!("{}", String::from_utf8(value.stderr).unwrap());
      }
    }
    Err(_err) => {
      log.error("error: {value}");
    }
  }
}

pub fn clone_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let config = config::load_config();

  let log = LabelLogger::default();

  let repo = use_flag("repo", Some('r'), &clier)
    .try_into()
    .map(|value: String| format!("https://{}", value))
    .unwrap_and(|_| {
      log.error("Repo flag doesn't exist");
      1
    });

  let project_dir = std::fs::metadata(&config.project_directory);

  match project_dir {
    Ok(value) => {
      if !value.is_dir() {
        log.error("project directory from config is not a directory");
        std::process::exit(1);
      }

      clone_project(
        &repo,
        &format!(
          "{}/{}",
          &config.project_directory,
          &repo.split('/').collect::<Vec<&str>>().last().unwrap()
        ),
      );

      ExitCode(0)
    }
    Err(err) => {
      if err.kind() == ErrorKind::NotFound {
        let path = Path::new(&config.project_directory);

        log.info("Creating dir...");
        std::fs::create_dir(path).unwrap();
        log.info("Rerun this script for it to work!");
        std::process::exit(0);
      } else {
        log.error("Unknwnw error");
      }

      ExitCode(1)
    }
  }
}
