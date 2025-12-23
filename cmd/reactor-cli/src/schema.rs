// reactor_cli/src/schema.rs
use clap::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CliSchema {
    pub name: String,
    pub args: Vec<ArgSchema>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArgSchema {
    pub name: String,
    pub help: String,
    pub required: bool,
    pub short: Option<char>,
    pub long: Option<String>,
}

impl CliSchema {
    pub fn from_command(cmd: &Command) -> Self {
        let args = cmd
            .get_arguments()
            .map(|a| ArgSchema {
                name: a.get_id().to_string(),
                help: a
                    .get_help()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| String::new()),
                required: a.is_required_set(),
                short: a.get_short(),
                long: a.get_long().map(|s| s.to_string()),
            })
            .collect();

        Self {
            name: cmd.get_name().to_string(),
            args,
        }
    }
}
