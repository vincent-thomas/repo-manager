use std::{
  io,
  process::{Command, Stdio},
};

use super::Searcher;

#[derive(Debug)]
pub struct Fzf;

impl Searcher for Fzf {
  fn search(&self, list: Vec<String>) -> Result<String, ()> {
    let search = list.join("\n");

    let echo_output = Command::new("echo")
      .arg(search) // Customize your input items here
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .stdout
      .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture echo stdout"))
      .unwrap();

    let command = Command::new("fzf")
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
          Ok(value)
        }
      }
      // TODO: error hantering med traits
      Err(_err) => Err(()),
    }
  }
}
