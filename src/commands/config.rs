use clier::{
  display::{label::LabelLogger, Displayer},
  hooks::use_double_dash,
  run::ExitCode,
  Clier, HasMeta, Runnable,
};

use crate::config::{self, Editors};

fn resolve_edit(editor: &str) -> Option<Editors> {
  match editor {
    "neovim" => Some(Editors::Neovim),
    "vscode" => Some(Editors::VSCode),
    _ => None,
  }
}

pub fn set_editor_config_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let log = LabelLogger::default();

  let editor = match clier.args.commands.get(2) {
    Some(editor) => resolve_edit(editor),
    None => {
      log.error("Editor not provided");
      return ExitCode(1);
    }
  };

  let gur_editor = editor.expect("Invalid editor");
  let mut config = config::load_config();
  config.editor = Some(gur_editor);
  config::write_config(config).expect("Failed to write");

  ExitCode(0)
}

pub fn set_dir_config_command(clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let path = use_double_dash(&clier).expect("Put the path after '--'");

  let mut config = config::load_config();
  config.project_directory = path;
  config::write_config(config).expect("Failed to write");

  ExitCode(0)
}
