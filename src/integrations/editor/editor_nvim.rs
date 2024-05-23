use std::process::Command;

use super::EditorOpen;
pub struct Neovim;

impl EditorOpen for Neovim {
  fn open(&self, dir: &str) -> Result<(), ()> {
    // TODO: Att inte denna variabeln inte används
    let command = Command::new("nvim")
      .args(["--cmd", &format!("cd {}", dir), dir])
      .spawn()
      .unwrap()
      .wait();

    Ok(())
  }
}
