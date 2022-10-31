// #[macro_use]
use log::{debug, info};
extern crate clap;



pub mod common;

use std::path::{PathBuf};
use clap::{Arg, Command};
use crate::common::logger;
use log::LevelFilter;

/// Holds all available user arguments
pub struct ParserOptions {
    blockchain_dir: PathBuf,  /* Path to directory where blk.dat files are stored */
}

fn main() {
    println!("Hello, world!");
    let options = match parse_args() {
        Ok(o) => o,
        Err(desc) => {
            info!("{}", desc);
            return;
        }
    };

    // Apply log filter based on verbosity
    // default to trace until verbosity arg is implemented
    _ = logger::init();
    // SimpleLogger::init(LogLevelFilter::Trace).expect("Unable to initialize logger");
    info!(target: "main", "Starting rust-btc-parser v{} ...", env!("CARGO_PKG_VERSION"));
    debug!(target: "main", "Using LogLevel {}", LevelFilter::Trace);
}

fn parse_args() -> Result<ParserOptions, String> {
    let matches = Command::new("rust-btc-parser")
        .author("Rory Shively")
        .version("0.0.1")
        .about("Parses BTC blockchain data")
        .arg(
            Arg::new("blockchain-dir")
            .short('d')
            .long("blockchain-dir")
            .help("Sets blockchain directory which contains blk.dat files (default: ~/.bitcoin/blocks)")
            .action(clap::ArgAction::Set)
        )
        .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
        .get_matches();

    let blockchain_dir: PathBuf;
    match matches.get_one::<String>("blockchain-dir") {
        Some(p) => blockchain_dir = PathBuf::from(p),
        None => return Err(String::from("must specify --blockchain-dir in args"))
    };

    Ok(ParserOptions {
        blockchain_dir: blockchain_dir,
    })
}
