use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    }
}


#[derive(Debug, Deserialize)]
pub struct DefaultProfile {
    pub browser: String,
    pub profile: String,
}


#[derive(Debug, Deserialize)]
pub struct BrowserProfile {
    pub browser: String,
    pub profile: String,
    pub urls: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct Config {
    pub browsers: HashMap<String, Vec<String>>,
    pub default: DefaultProfile,
    pub profiles: Vec<BrowserProfile>,
}


impl Config {
    pub fn from_file(p: &Path) -> Result<Config> {
        let mut f = try!(File::open(p));
        let mut buf = String::new();

        try!(f.read_to_string(&mut buf));

        let config = try!(serde_yaml::from_str(&buf));
        Ok(config)
    }
}
