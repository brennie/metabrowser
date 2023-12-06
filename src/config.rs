use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::Deserialize;

/// A browser and the profile it should open with.
#[derive(Deserialize)]
pub struct BrowserProfile {
    /// The name of the browser, as defined in the `browsers` section.
    pub browser: String,

    /// The name of the profile.
    ///
    /// This will be subsituted into the `{profile}` parameter to the browser.
    pub profile: String,
}

/// A set of UR
#[derive(Deserialize)]
pub struct Rule {
    /// The browser and profile to open the associated URLs with.
    pub open_in: BrowserProfile,

    /// A set of URL patterns (supporting wildcards) that should open in the
    /// associated browser and profile.
    pub url_patterns: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    /// A mapping of browsers and the command-line arguments to start them.
    ///
    /// The special token `{profile}` in the command-line will be replaced with
    /// a profile.
    pub browsers: HashMap<String, Vec<String>>,

    /// The default browser and profile to use when opening a URL that does not
    /// match any rules.
    pub default: BrowserProfile,

    /// Rules for determining which URLs open in which browser profiles.
    pub rules: Vec<Rule>,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.browsers.get(&self.default.browser).is_none() {
            return Err(anyhow!(
                "Browser definition for '{}' is missing",
                self.default.browser
            ));
        }

        for rule in &self.rules {
            if self.browsers.get(&rule.open_in.browser).is_none() {
                return Err(anyhow!(
                    "Browser definition for '{}' is missing",
                    rule.open_in.browser
                ));
            }
        }

        for (browser, command_template) in &self.browsers {
            if command_template.is_empty() {
                return Err(anyhow!(
                    "Browser definition for '{}' has empty command template",
                    browser
                ));
            }
            if !command_template[1..]
                .iter()
                .any(|part| part.contains("{profile}"))
            {
                return Err(anyhow!(
                    "Browser definition for '{}' is missing {{profile}} in command template",
                    browser
                ));
            }
            if !command_template[1..]
                .iter()
                .any(|part| part.contains("{url}"))
            {
                return Err(anyhow!(
                    "Browser definition for '{}' is missing {{profile}} in command template",
                    browser
                ));
            }
        }

        Ok(())
    }
}
