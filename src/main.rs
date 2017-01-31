#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;

use self::config::{Config, Error, ErrorKind};
use std::path::Path;
use std::io::{stderr, Write};
use std::env::home_dir;


fn main() {
    let result = home_dir()
        .ok_or(Error::from(ErrorKind::NoHomeError))
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
