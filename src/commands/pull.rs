use std::{
  io::ErrorKind,
  path::Path,
  process::{Command, Stdio},
};

use clier::{display::Displayer, hooks::use_flag, run::ExitCode, Clier, HasMeta, Runnable};

use crate::{config, utils::UnwrapAnd};

pub fn clone_project(repo_url: &str, dir: &str) {
  let log_err = Displayer::Error {};
  let log_success = Displayer::Success {};
  let log_info = Displayer::Info {};

  log_info.write("Cloning...");

  let git_clone_command = Command::new("git")
    .args(["clone", repo_url, dir])
    .stdin(Stdio::piped())
    .stderr(Stdio::piped())
    .output();

  match git_clone_command {
    Ok(value) => {
      if value.status.success() {
        log_success.write("Pulled project!");
      } else {
        log_err.write_err("Error writing project:");
        eprintln!("{}", String::from_utf8(value.stderr).unwrap());
      }
    }
    Err(_err) => {
      log_err.write_err("error: {value}");
    }
  }
}

pub fn clone_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let config = config::load_config();

  let log_err = Displayer::Error {};
  let log_info = Displayer::Info {};

  let repo = use_flag("repo", Some('r'), &clier)
    .try_into()
    .map(|value: String| format!("https://{}", value))
    .unwrap_and(|_| {
      log_err.write_err("Repo flag doesn't exist");
      1
    });

  dbg!(&repo);

  let project_dir = std::fs::metadata(&config.project_directory);

  match project_dir {
    Ok(value) => {
      if !value.is_dir() {
        log_err.write_err("project directory from config is not a directory");
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

        log_info.write("Creating dir...");
        std::fs::create_dir(path).unwrap();
        log_info.write("Rerun this script for it to work!");
        std::process::exit(0);
      } else {
        log_err.write_err("Unknwnw error");
      }

      ExitCode(1)
    }
  }
}
