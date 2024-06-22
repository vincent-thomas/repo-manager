use std::process::{Command, Stdio};

use serde::Deserialize;

use crate::integrations::{CheckHealth, HealthError};

#[derive(Debug)]
pub struct GhSource;

impl Default for GhSource {
  fn default() -> Self {
    GhSource
  }
}

#[derive(Deserialize)]
struct RawRepoGhCli {
  name: String,
  url: String,
}

/* impl Source for GhSource {
  fn list_repos(&self, _config: &Configuration) -> Vec<Repository> {
    let command = Command::new("gh")
      .args(["repo", "list", "--json", "name,url"])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .wait_with_output();

    let test = command.unwrap().stdout;

    let string_nice = String::from_utf8(test).unwrap();

    let formatted: Vec<RawRepoGhCli> = serde_yaml::from_str(&string_nice).unwrap();

    formatted
      .iter()
      .map(|value| Repository {
        repo_name: value.name.clone(),
        link: super::Link::Remote(value.url.clone()),
        display_name: format!("{} {}", "", &value.name),
      })
      .collect()
    /*     string_nice
    .split('\n')
    .map(|value| )
    .collect::<Vec<String>>() */
  }
} */

impl CheckHealth for GhSource {
  fn checkhealth(&self) -> Result<(), crate::integrations::HealthError>
  where
    Self: Sized,
  {
    Command::new("gh")
      .arg("--version")
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status()
      .map(|_| ())
      .map_err(|_| HealthError {
        severity: crate::integrations::HealthSeverity::Error,
        messages: "path 'gh' is not installed".into(),
      })
  }
}
