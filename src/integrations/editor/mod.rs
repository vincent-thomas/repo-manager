use crate::config::{Configuration, Editors};

pub mod editor_nvim;
pub mod editor_vscode;

pub trait EditorOpen {
  fn open(&self, dir: &str) -> Result<(), ()>;
}

pub fn get_editor<'a>(config: &Configuration) -> Option<&'a dyn EditorOpen> {
  let editor: &'a dyn EditorOpen = match config.editor? {
    Editors::Neovim => &editor_nvim::Neovim,
    Editors::VSCode => &editor_vscode::VSCode,
  };

  Some(editor)
}
