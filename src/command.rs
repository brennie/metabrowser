use std::process::Command;

use anyhow::Result;
use clap::Args;

#[cfg(windows)]
pub use crate::command::windows::{install, uninstall, InstallOptions};
use crate::config::{BrowserProfile, Config};
use crate::url::url_patterns_to_regex;

#[derive(Args)]
pub struct OpenOptions {
    #[arg(long)]
    /// Check which browser would open the given URL.
    check: bool,

    #[arg()]
    /// The URL to open.
    url: String,
}

pub fn open_url(options: &OpenOptions) -> Result<()> {
    let config = Config::load()?;
    let mut open_in = &config.default;

    for rule in &config.rules {
        let re = match url_patterns_to_regex(&rule.url_patterns) {
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

#[cfg(windows)]
mod windows {
    use std::env::current_exe;
    use std::ffi::OsString;
    use std::process::Command;

    use anyhow::Result;
    use clap::Args;
    use winreg::enums::*;
    use winreg::RegKey;

    #[derive(Args)]
    pub struct InstallOptions {
        #[arg(long)]
        pub set_default: bool,
    }

    pub fn install(options: &InstallOptions) -> Result<()> {
        {
            let class_root = RegKey::predef(HKEY_CURRENT_USER).open_subkey("SOFTWARE\\Classes")?;

            let metabrowser_class = class_root.create_subkey("metabrowserHTML")?.0;
            metabrowser_class.set_value("", &"metabrowser HTML Document")?;
            metabrowser_class.set_value("AppUserModelId", &"metabrowser")?;

            let application = metabrowser_class.create_subkey("Application")?.0;
            application.set_value("", &"metabrowser HTML document")?;
            application.set_value("AppUserModelId", &"metabrowser")?;
            application.set_value("ApplicationName", &"metabrowser")?;
            application.set_value(
                "ApplicationDescription",
                &"Open URLs in specific browsers based on rules",
            )?;
            application.set_value("ApplicationCompany", &"Barret Rennie")?;

            let shell_open_key = metabrowser_class.create_subkey("shell\\open\\command")?.0;
            let open_value = {
                const PREFIX: &str = r#"""#;
                const SUFFIX: &str = r#"" open "%1""#;

                let program_path = current_exe()?;

                let mut buf = OsString::new();
                buf.push(PREFIX);
                buf.push(program_path);
                buf.push(SUFFIX);
                buf
            };
            shell_open_key.set_value("", &open_value)?;

            let html_key = class_root.create_subkey(".html\\OpenWithProgids")?.0;
            html_key.set_value("metabrowserHTML", &"")?;
        }

        {
            let client = RegKey::predef(HKEY_CURRENT_USER)
                .create_subkey("Software\\Clients\\StartMenuInternet\\metabrowserHTML")?
                .0;

            {
                let caps = client.create_subkey("Capabilities")?.0;
                caps.set_value("ApplicationName", &"metabrowser")?;
                caps.set_value(
                    "ApplicationDescription",
                    &"Open URLs in specific browsers based on rules",
                )?;
                {
                    let url_associations = caps.create_subkey("URLAssociations")?.0;
                    for scheme in &["http", "https"] {
                        url_associations.set_value(scheme, &"metabrowserHTML")?;
                    }
                }
            }

            {
                let registered_applications_key = RegKey::predef(HKEY_CURRENT_USER)
                    .open_subkey_with_flags("SOFTWARE\\RegisteredApplications", KEY_ALL_ACCESS)?;
                registered_applications_key.set_value(
                    "metabrowser",
                    &"Software\\Clients\\StartMenuInternet\\metabrowserHTML\\Capabilities",
                )?;
            }
        }

        if options.set_default {
            println!("Opening control panel so you can set your default browser");
            Command::new("control.exe")
                .arg("/name")
                .arg("Microsoft.DefaultPrograms")
                .arg("/page")
                .arg("pageDefaultProgram")
                .spawn()?;
        }

        Ok(())
    }

    pub fn uninstall() -> Result<()> {
        let _ = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags("SOFTWARE\\Classes", KEY_ALL_ACCESS)?
            .delete_subkey_all("metabrowserHTML");

        let _ = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags("Software\\Clients\\StartMenuInternet", KEY_ALL_ACCESS)?
            .delete_subkey_all("metabrowserHTML");

        let _ = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags("SOFTWARE\\RegisteredApplications", KEY_ALL_ACCESS)?
            .delete_value("metabrowser");

        Ok(())
    }
}
