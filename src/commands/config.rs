use clier::{
  display::{label::LabelLogger, Displayer},
  hooks::use_double_dash,
  run::ExitCode,
  Clier, HasMeta, Runnable,
};

use crate::config::{self};

fn resolve_edit(editor: &str) -> Option<config::Editors> {
  match editor {
    "neovim" => Some(config::Editors::Neovim),
    "vscode" => Some(config::Editors::VSCode),
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
  config.editor = gur_editor;
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

pub fn generate_config_command(_: Clier<HasMeta, Runnable>) -> ExitCode {
  let default_config = config::Configuration::default();

  // TODO: detta
  let _ = config::write_config(default_config);

  ExitCode(0)
}
