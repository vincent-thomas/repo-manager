use std::process::{Command, Stdio};

use serde::Deserialize;

use super::{Repository, Source};

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

impl Source for GhSource {
  fn list() -> Vec<Repository> {
    let command = Command::new("gh")
      .args(["repo", "list", "--json", "name,url"])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .wait_with_output();

    let test = command.unwrap().stdout;

    let string_nice = String::from_utf8(test).unwrap();

    let formatted: Vec<RawRepoGhCli> = serde_json::from_str(&string_nice).unwrap();

    formatted
      .iter()
      .map(|value| Repository {
        repo_id: value.name.clone(),
        link: super::Link::Url(value.url.clone()),
        display_name: format!("{} {}", "", &value.name),
      })
      .collect()
    /*     string_nice
    .split('\n')
    .map(|value| )
    .collect::<Vec<String>>() */
  }
}
