use std::process::Command;

use super::EditorOpen;
pub struct VSCode;

impl EditorOpen for VSCode {
  fn open(&self, dir: &str) -> Result<(), ()> {
    // TODO: inte använt
    let _command = Command::new("code").args([dir]).spawn().unwrap().wait();

    Ok(())
  }
}
