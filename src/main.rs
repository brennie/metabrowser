#[macro_use]
extern crate error_type;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;

use self::config::{Config, Error};
use std::path::Path;
use std::io::{stderr, Write};
use std::env::home_dir;


fn main() {
    let result = home_dir()
        .ok_or(Error::GenericError(String::from("could not locate home directory.")))
        .map(|p| Path::new(&p).join(".metabrowser.yaml"))
        .and_then(|p| Config::from_file(&p));

    match result {
        Ok(c) => println!("{:?}", c),
        Err(e) => {
            writeln!(&mut stderr(),
                     "Could not open metabrowser configuration file: {}",
                     e)
                .unwrap();
        }
    }
}
