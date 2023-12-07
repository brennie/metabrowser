mod command;
mod config;
mod url;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[cfg(windows)]
use crate::command::{install, uninstall, InstallOptions};
use crate::command::{open_url, OpenOptions};

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

    /// Install metabrowser as a web browser.
    #[cfg(windows)]
    Install(InstallOptions),

    /// Uninstall metabrowser as a web browser.
    #[cfg(windows)]
    Uninstall,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let subcommand = options
        .subcommand
        .unwrap_or_else(|| SubCommand::Open(options.open_options.unwrap()));

    match subcommand {
        SubCommand::Open(open_options) => open_url(&open_options)?,

        #[cfg(windows)]
        SubCommand::Install(install_options) => install(&install_options)?,

        #[cfg(windows)]
        SubCommand::Uninstall => uninstall()?,
    };

    Ok(())
}
