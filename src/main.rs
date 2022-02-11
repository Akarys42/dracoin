mod cli;
mod logging;

extern crate clap;

use clap::Parser;
#[allow(unused_imports)]
use log::{debug, warn, LevelFilter};

use cli::Cli;
use logging::init_cli_logging;

fn main() {
    let cli = Cli::parse();

    // Start logging in debug if the verbose flag is present.
    init_cli_logging(if cli.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    })
    .expect("Failed to initialize logging.");

    debug!("Logging successfully initialized.");

    #[cfg(debug_assertions)]
    warn!("Debug build. Do not use in production.");
}
