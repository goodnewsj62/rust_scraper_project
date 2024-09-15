use std::path::PathBuf;

use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    DATABASE_URI: String,

    #[arg(short, long, value_name = "FILE")]
    envfile: Option<PathBuf>,
}

pub struct Config {
    pub db_uri: String,
}

impl Config {
    pub fn build() -> Self {
        let cli = Cli::parse();

        let regex = Regex::new("postgresql://.+:.+@.+/.+$").unwrap();

        if !regex.is_match(&cli.DATABASE_URI) {
            panic!("please set a valid DATABASE_URI variable")
        }

        Config {
            db_uri: cli.DATABASE_URI,
        }
    }
}
