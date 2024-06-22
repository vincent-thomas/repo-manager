use std::{
  io,
  process::{Command, Stdio},
};

use crate::integrations::{srcs::ListItem, CheckHealth};

use super::Searcher;

#[derive(Debug)]
pub struct Fzf;

impl Searcher for Fzf {
  fn search<'a>(&self, list: &'a [ListItem], initial_search: &str) -> Result<&'a ListItem, ()> {
    let search = list
      .iter()
      .map(|value| match value.icon {
        Some(icon) => format!("{} {}", icon, value.name),
        None => format!("{}", value.name),
      })
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
        dbg!(&value);
        let value = String::from_utf8(value.stdout)
          .expect("Command output not utf8")
          .replace('\n', "");

        let without_icon = value.split(' ').last().ok_or(())?;

        if without_icon.is_empty() {
          Err(())
        } else {
          let test = list.iter().find(|to_check| to_check.name == without_icon);

          test.ok_or(())
        }
      }
      // TODO: error hantering med traits
      Err(_err) => {
        panic!("I don't know {:?}", _err);
        Err(())
      }
    }
  }
}

impl CheckHealth for Fzf {
  fn checkhealth(&self) -> Result<(), crate::integrations::HealthError> {
    Command::new("fzf")
      .arg("--version")
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status()
      .map(|_| ())
      .map_err(|_| crate::integrations::HealthError {
        severity: crate::integrations::HealthSeverity::Error,
        messages: "program 'fzf' is not installed".into(),
      })
  }
}
