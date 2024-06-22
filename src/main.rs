use clier::{run::ExitCode, CliMeta, Clier, CmdMeta, Commands};
use gitm::commands::{
  config::{generate_config_command, set_dir_config_command, set_editor_config_command},
  doctor::doctor_command,
  goto::goto_command,
  pull::clone_command,
  remove::remove_command,
};

const NAME: &str = env!("CARGO_BIN_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn meta() -> CliMeta {
  CliMeta {
    name: NAME.to_string(),
    description: DESCRIPTION.to_string(),
    version: Some(VERSION.to_string()),
    usage: Some("<command> [--flags=value]".to_string()),
  }
}

fn main() -> ExitCode {
  let cli = Clier::parse().meta(meta());
  let app = cli.runnable(vec![
    Commands::Command {
      meta: CmdMeta::new("clone", "Clone a repo and put it where you want"),
      handler: clone_command,
    },
    Commands::Command {
      meta: CmdMeta::new("remove", "Removes a repo from your file system"),
      handler: remove_command,
    },
    Commands::Command {
      meta: CmdMeta::new("doctor", "Runs checks"),
      handler: doctor_command,
    },
    Commands::Command {
      meta: CmdMeta::new(
        "go",
        "Goes to the current project and optionally starts your wanted editor",
      ),
      handler: goto_command,
    },
    Commands::Collection(clier::CmdCollection {
      meta: CmdMeta::new("config", "Can change config file in the terminal"),
      children: Box::from([
        Commands::Command {
          meta: CmdMeta::new("editor", "Can change the editor"),
          handler: set_editor_config_command,
        },
        Commands::Command {
          meta: CmdMeta::new("gen", "Generates default config"),
          handler: generate_config_command,
        },
        Commands::Command {
          meta: CmdMeta::new(
            "path",
            "The project directory where your projects should live",
          ),
          handler: set_dir_config_command,
        },
      ]),
    }),
  ]);
  app.run()
}
