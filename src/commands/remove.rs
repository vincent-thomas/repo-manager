use std::{
  fs,
  io::{self, Write},
  path::Path,
};

use clier::{
  display::{label::LabelLogger, Displayer},
  hooks::{use_flag, FlagError},
  run::ExitCode,
  Clier, HasMeta, Runnable,
};

use crate::config;

fn handle_err(err: FlagError) {
  let log = LabelLogger::default();
  match err {
    FlagError::Unexisting => {
      log.error("flag 'name' is required");
    }
    FlagError::InvalidFormat => {
      log.error("flag 'name' is required to be a string");
    }
    _ => {
      log.error("flag 'name' is invalid");
    }
  };
}

fn get_input_incase_not_yes() -> Option<bool> {
  let mut buffer = String::new();

  io::stdin().read_line(&mut buffer).ok()?;

  match buffer.trim().to_lowercase().as_str() {
    "yes" | "y" => Some(true),
    "no" | "n" => Some(false),
    _ => None,
  }
}
fn ensure_put_folder(name: &str) {
  let path = Path::new(name);

  if !path.exists() {
    dbg!("path doesnt exist");
  }
}

pub fn remove_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let log = LabelLogger::default();
  let config = config::load_config();

  let project_name: String = match use_flag("name", None, &clier).try_into() {
    Ok(value) => value,
    Err(err) => {
      handle_err(err);
      return ExitCode(1);
    }
  };

  let project_dir = format!("{}/{project_name}", &config.project_directory);

  ensure_put_folder(&project_dir);

  let yes_flag: Option<bool> = use_flag("yes", Some('y'), &clier).try_into().ok();

  let do_it: Option<bool> = match yes_flag {
    Some(value) => Some(value),
    None => {
      log.warn("THIS WILL REMOVE ALL FILES AND DIRECTORIES UNDER THIS PATH");

      print!("Are you sure you want to delete this directory? (yes/no or y/n): ");
      let mut stdout = std::io::stdout();
      stdout.flush().unwrap();
      get_input_incase_not_yes()
    }
  };

  if do_it.is_some_and(|value| !value) || do_it.is_none() {
    log.error("yes was not provided");
    return ExitCode(1);
  }

  match fs::remove_dir_all(&project_dir) {
    Ok(_) => {
      log.info(&format!("Directory {} has been removed", project_dir));
    }
    Err(err) => {
      log.error(&format!("unknown err: {err:#?}"));
    }
  }

  ExitCode(0)
}
