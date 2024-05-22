use std::{
  io,
  process::{Command, Stdio},
};

use crate::integrations::srcs::Repository;

use super::Searcher;

#[derive(Debug)]
pub struct Pick;

impl Searcher for Pick {
  /// TODO: _initial
  fn search(&self, list: Vec<Repository>, _initial: &str) -> Result<Repository, ()> {
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

    let command = Command::new("pick")
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
          let thing_to_return = list.iter().find(|to_check| to_check.display_name == value);

          thing_to_return.ok_or(()).cloned()
        }
      }
      // TODO: error hantering med traits
      Err(_err) => Err(()),
    }
  }
}
