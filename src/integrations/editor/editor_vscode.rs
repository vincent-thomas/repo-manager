use std::process::{Command, Stdio};

use crate::integrations::{self, CheckHealth, HealthError};

use super::EditorOpen;
pub struct VSCode;

impl EditorOpen for VSCode {
  fn open(&self, dir: std::path::PathBuf) -> Result<(), ()> {
    // TODO: inte använt
    let _command = Command::new("code").args([dir]).spawn().unwrap().wait();

    Ok(())
  }
}

impl CheckHealth for VSCode {
  fn checkhealth(&self) -> Result<(), integrations::HealthError> {
    Command::new("code")
      .arg("--version")
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status()
      .map(|_| ())
      .map_err(|_| HealthError {
        severity: integrations::HealthSeverity::Error,
        messages: "the program 'code' is not available in $PATH".into(),
      })
  }
}
