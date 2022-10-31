use log::{LevelFilter, debug, info, warn, error};
use simplelog::{TermLogger, TerminalMode, Config, ColorChoice};

// extern crate clap;
use clap::{Arg, Command};

use std::path::{PathBuf};

/// Holds all available user arguments
pub struct ParserOptions {
    blockchain_dir: PathBuf,  /* Path to directory where blk.dat files are stored */
}

fn main() {
    // Setup logging
    // more logging options found at https://crates.io/crates/log
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    ).unwrap();

    let options = match parse_args() {
        Ok(o) => o,
        Err(desc) => {
            info!("{}", desc);
            return;
        }
    };

    info!("Running parser in blockchain directory: [{}]", options.blockchain_dir.display());
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
