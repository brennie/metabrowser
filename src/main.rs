mod config;
mod url;

use std::fs::File;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use directories::ProjectDirs;

use crate::config::{BrowserProfile, Config};

#[derive(Parser)]
#[command(about)]
pub struct Options {
    #[arg(long)]
    /// Check which browser would open the given URL.
    check: bool,

    #[arg()]
    /// The URL to open.
    url: String,
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

    let mut open_in = &config.default;

    for rule in &config.rules {
        let re = match url::to_regex(&rule.url_patterns) {
            Some(re) => re,
            None => continue,
        };

        if re.is_match(&options.url) {
            open_in = &rule.open_in;
            break;
        }
    }

    let mut command = build_command(&config.browsers[&open_in.browser], open_in, &options.url)?;

    if options.check {
        println!(
            "{} {}",
            command.get_program().to_string_lossy(),
            command
                .get_args()
                .map(|s| s.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        command.spawn()?;
    }

    Ok(())
}

fn build_command(
    command_template: &[String],
    open_in: &BrowserProfile,
    url: &str,
) -> Result<Command> {
    use std::borrow::{Borrow, Cow};

    assert!(!command_template.is_empty());

    let mut cmd = Command::new(&command_template[0]);
    for arg in &command_template[1..] {
        let mut arg = Cow::from(arg);
        if arg.contains("{profile}") {
            arg = Cow::from(arg.replace("{profile}", &open_in.profile));
        }
        if arg.contains("{url}") {
            arg = Cow::from(arg.replace("{url}", url));
        }
        cmd.arg::<&str>(arg.borrow());
    }

    Ok(cmd)
}
