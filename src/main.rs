mod commands;
mod config;
mod utils;

use clier::{run::ExitCode, CliMeta, Clier, CmdMeta, Commands};
use commands::pull::clone_command;

const NAME: &str = env!("CARGO_BIN_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn meta() -> CliMeta {
    CliMeta {
        name: NAME.to_string(),
        description: DESCRIPTION.to_string(),
        version: Some((0, 0, 0)),
        usage: Some("<command> [--flags=value]".to_string()),
    }
}

fn main() -> ExitCode {
    let cli = Clier::parse().meta(meta());
    let app = cli.runnable(vec![Commands::Command {
        meta: CmdMeta::new("pull", "Clone a repo and put it where you want"),
        handler: clone_command,
    }]);
    app.run()
}
