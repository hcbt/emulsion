mod config;

use std::path::PathBuf;

use clap::{arg, value_parser, command};

fn main() {
    let matches = command!()
        .arg(arg!(--config <FILE>).required(true).value_parser(value_parser!(PathBuf)))
        .arg(arg!(--recipes <FILE>).required(false).value_parser(value_parser!(PathBuf)))
        .get_matches();

    if let Some(config_file) = matches.get_one::<PathBuf>("config") {
        let contents = std::fs::read_to_string(config_file).expect("failed to read config file");
        let config: config::Config = serde_yaml::from_str(&contents).expect("failed to parse config file");
        println!("{:#?}", config);
    }
}