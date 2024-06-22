use clier::{
  display::{label::LabelLogger, Displayer},
  run::ExitCode,
  Clier, HasMeta, Runnable,
};

use crate::integrations::{
  editor::{editor_nvim::Neovim, editor_vscode::VSCode},
  search::{fzf::Fzf, pick::Pick},
  CheckHealth, HealthSeverity,
};

pub fn doctor_command(_clier: Clier<HasMeta, Runnable>) -> ExitCode {
  let thing: &[&dyn CheckHealth] = &[&VSCode, &Neovim, &Pick, &Fzf];
  let log = LabelLogger::default();

  for things in thing {
    let value = things.checkhealth();

    if let Err(err) = value {
      match err.severity {
        HealthSeverity::Error => log.error(&err.messages),
        HealthSeverity::Warn => log.warn(&err.messages),
      }
    }
  }

  ExitCode(1)
}
