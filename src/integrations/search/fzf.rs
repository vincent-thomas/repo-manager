use std::{
  io,
  process::{Command, Stdio},
};

use crate::integrations::srcs::Repository;

use super::Searcher;

#[derive(Debug)]
pub struct Fzf;

impl Searcher for Fzf {
  fn search(&self, list: Vec<Repository>, initial_search: &str) -> Result<Repository, ()> {
    let search = list
      .iter()
      .map(|value| value.display_name.clone())
      .collect::<Vec<String>>()
      .join("\n");

    let echo_output = Command::new("echo")
      .arg(search) // Customize your input items here
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .stdout
      .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture echo stdout"))
      .unwrap();

    let mut command_builder = Command::new("fzf");

    if !initial_search.is_empty() {
      command_builder.args(["-q", initial_search]);
    }

    let command = command_builder
      .stdin(echo_output)
      .stdout(Stdio::piped())
      .stderr(Stdio::inherit())
      .spawn()
      .unwrap();

    match command.wait_with_output() {
      Ok(value) => {
        let value = String::from_utf8(value.stdout)
          .expect("Command output not utf8")
          .replace('\n', "");

        if value.is_empty() {
          Err(())
        } else {
          let test = list.iter().find(|to_check| to_check.display_name == value);

          test.ok_or(()).cloned()
        }
      }
      // TODO: error hantering med traits
      Err(_err) => Err(()),
    }
  }
}
