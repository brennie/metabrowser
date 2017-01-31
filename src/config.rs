use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::result;

use serde_yaml;


error_type! {
    #[derive(Debug)]
    pub enum Error {
        GenericError(String) {
            desc (e) &**e;
        },
        IOError(io::Error) { cause; },
        ParseError(serde_yaml::Error) {
            cause;
        },
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


pub type Result = result::Result<Config, Error>;
impl Config {
    pub fn from_file(p: &Path) -> Result {
        let mut f = try!(File::open(p));
        let mut buf = String::new();

        try!(f.read_to_string(&mut buf));

        let config = try!(serde_yaml::from_str(&buf));
        Ok(config)
    }
}
