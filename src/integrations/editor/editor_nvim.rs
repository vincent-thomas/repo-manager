use std::{
  path::PathBuf,
  process::{Command, Stdio},
};

use crate::integrations::{self, CheckHealth, HealthError};

use super::EditorOpen;
pub struct Neovim;

impl EditorOpen for Neovim {
  fn open(&self, dir: PathBuf) -> Result<(), ()> {
    // TODO: Bättre jobb
    let _ = Command::new("nvim")
      .args([
        "--cmd",
        &format!("cd {}", dir.to_str().expect("path is not valid utf-8")),
        dir.to_str().expect("path is not valid utf-8"),
      ])
      .spawn()
      .unwrap()
      .wait();

    Ok(())
  }
}

impl CheckHealth for Neovim {
  fn checkhealth(&self) -> Result<(), integrations::HealthError> {
    Command::new("nvim")
      .arg("--version")
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status()
      .map(|_| ())
      .map_err(|_| HealthError {
        severity: integrations::HealthSeverity::Error,
        messages: "the program 'nvim' is not available in $PATH".into(),
      })
  }
}
