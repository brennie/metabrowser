mod command;
mod config;
mod url;

use std::fs::File;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;

use crate::command::{open_url, OpenOptions};
use crate::config::Config;

#[derive(Parser)]
#[command(about)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Options {
    #[command(subcommand)]
    subcommand: Option<SubCommand>,

    #[command(flatten)]
    open_options: Option<OpenOptions>,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Open a URL.
    Open(OpenOptions),
}

fn main() -> Result<()> {
    let options = Options::parse();

    let config_path = ProjectDirs::from("ca", "brennie", "metabrowser")
        .ok_or_else(|| anyhow!("Could not get project dirs"))?
        .config_dir()
        .join("metabrowser.yml");

    let f = File::open(&config_path).with_context(|| {
        format!(
            "Could not open metabrowser config at: {}",
            config_path.display()
        )
    })?;
    let config = serde_yaml::from_reader::<_, Config>(f).with_context(|| {
        format!(
            "Could not parse metabrowser config at: {}",
            config_path.display()
        )
    })?;

    config.validate()?;

    let subcommand = options
        .subcommand
        .unwrap_or_else(|| SubCommand::Open(options.open_options.unwrap()));

    match subcommand {
        SubCommand::Open(open_options) => open_url(&config, &open_options)?,
    };

    Ok(())
}
