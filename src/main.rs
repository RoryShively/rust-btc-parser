use std::process;

use log::{LevelFilter, debug, info, warn, error};
use simplelog::{TermLogger, TerminalMode, Config, ColorChoice};

// extern crate clap;
use clap::{Arg, Command};

use std::path::{PathBuf};

pub mod blockchain;


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
            error!("argument parsing err: {}", desc);
            process::exit(1);
        }
    };

    info!("Starting rusty-blockparser v{} ...", env!("CARGO_PKG_VERSION"));
    info!("Running parser in blockchain directory: [{}]", options.blockchain_dir.display());



    // let chain_storage = match ChainStorage::new(&options) {
    //     Ok(storage) => storage,
    //     Err(e) => {
    //         error!(
    //             "Cannot load blockchain from: '{}'. {}",
    //             options.borrow().blockchain_dir.display(),
    //             e
    //         );
    //         process::exit(1);
    //     }
    // };
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
        None => return Err(String::from("must specify --blockchain-dir"))
    };

    Ok(ParserOptions {
        blockchain_dir: blockchain_dir,
    })
}
