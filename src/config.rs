use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use regex;
use regex::{Regex, RegexBuilder};
use serde_yaml;


error_chain! {
    foreign_links {
        ParseError(::serde_yaml::Error);
        IOError(::std::io::Error);
    }

    errors {
        NoHomeError {
            description("could not locate home directory")
        }

        BadEntryError(browser: String, profile: String) {
            description("bad configuration")
            display("bad configuration: no such browser '{}' defined for profile '{}'",
                    browser,
                    profile)
        }
    }
}


#[doc(hidden)]
/// A regex that is equivalent to the "*." wildcard pattern.
///
/// This regular expression matches any subdomains (and subdomains of subdomains, etc.). It replaces
/// the "*." in entries such as "*.example.com"
static WC_REPLACEMENT: &'static str = r"(?:.+\.)?";

#[doc(hidden)]
/// The prefix for the generated regular expression.
///
/// This prefix will make the regular expression match both HTTP and HTTPs URLs.
static RE_PREFIX: &'static str = r"^https?://(?:";

#[doc(hidden)]
/// The suffix for the generated regular expression.
///
/// This is the corresponding closing parenthesis for the opening parenthesis in `RE_PREFIX`.
static RE_SUFFIX: char = ')';


#[doc(hidden)]
/// Transform a string into a regular expression that will match that URL.
///
/// URLs of the form "*.example.com" will match any subdomains of "example.com" in addition to
/// "example.com".
fn escape_url(url: &str) -> String {
    if url.starts_with("*.") {
        let rest = url.split_at(2).1;
        let mut escaped = String::with_capacity(WC_REPLACEMENT.len() + url.len() - 2);

        escaped.push_str(WC_REPLACEMENT);
        escaped.push_str(&regex::escape(&rest));
        escaped
    } else {
        regex::escape(&url)
    }
}


#[doc(hidden)]
/// Transform a list of URLs to a regular expression that matches them.
///
/// URLs should not include the scheme: the resulting regular expression will match both HTTP and
/// HTTPs URLs.
///
/// If the list is empty, no regular expression is returned.
fn urls_to_regex(urls: &Vec<String>) -> Option<Regex> {
    if urls.is_empty() {
        None
    } else {
        let re_body = urls.iter()
            .map(|url| escape_url(url))
            .collect::<Vec<_>>()
            .join("|");

        let mut pattern = String::with_capacity(RE_PREFIX.len() + re_body.len() + 1);
        pattern.push_str(RE_PREFIX);
        pattern.push_str(&re_body);
        pattern.push(RE_SUFFIX);

        // build() returns a ::std::result::Result<regex::Regex, regex::Error>, but panicing is not
        // possible because we escape all user input (and we therefore cannot generate an invalid
        // regular expression).
        Some(RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()
            .unwrap())
    }
}


#[derive(Debug, Deserialize)]
/// A default browser and profile combination.
pub struct DefaultProfile {
    /// The default browser.
    pub browser: String,

    /// The default profile to use for the browser.
    pub profile: String,
}


#[derive(Debug, Deserialize)]
/// A configuration entry.
///
/// Each entry maps a regular expression to a browser and profile. If a URL matches the given
/// regular expression it will be launched using the corresponding browser and profile.
pub struct Entry {
    /// The browser to use.
    ///
    /// This must be defined in the list of browsers in the parent [`Config`](struct.Config.html)
    /// struct.
    pub browser: String,

    /// The profile to use.
    pub profile: String,

    /// The list of URLs that this profile matches.
    ///
    /// This list is used to generate the regular expression.
    pub urls: Vec<String>,

    /// The regular expression to match against.
    ///
    /// If a URL matches this regular expression, the corresponding browser will be launched with
    /// the corresponding profile.
    ///
    /// If the list of URLs is empty, this will be `None`.
    #[serde(skip_deserializing)]
    pub regex: Option<Regex>,
}


#[derive(Debug, Deserialize)]
/// Metabrowser configuration.
///
/// The configuration specifies the set of browsers and how they can be launched, as well as the
pub struct Config {
    /// A mapping of browser names to their command line parameters for launching with a specified
    /// profile.
    pub browsers: HashMap<String, Vec<String>>,

    /// The default browser and profile to use for URLs that do not match any regular expressions in
    /// the list of profiles.
    pub default: DefaultProfile,

    /// The specified browser-profile pairs and their corresponding regular expressions.
    #[serde(rename="profiles")]
    pub entries: Vec<Entry>,
}


impl Config {
    /// Read the configuration from the file.
    pub fn from_file(p: &Path) -> Result<Config> {

        let mut buf = String::new();
        let mut f = try!(File::open(p));
        try!(f.read_to_string(&mut buf));

        let mut config = try!(serde_yaml::from_str::<Self>(&buf)
            .map_err(Error::from)
            .and_then(Config::validate));

        for entry in &mut config.entries {
            entry.regex = urls_to_regex(&entry.urls);
        }

        Ok(config)
    }

    #[doc(hidden)]
    /// Validate a parsed configuration.
    ///
    /// This function will ensure that all entries match up with the list of available browsers.
    fn validate(mut self) -> Result<Self> {
        for i in 0..self.entries.len() {
            if !self.browsers.contains_key(&self.entries[i].browser) {
                let browser_profile = self.entries.swap_remove(i);
                return Err(Error::from(ErrorKind::BadEntryError(browser_profile.browser,
                                                                browser_profile.profile)));
            }
        }

        Ok(self)
    }
}
