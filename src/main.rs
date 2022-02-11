mod logging;
mod cli;

extern crate clap;

use log::{debug, warn, LevelFilter};
use clap::Parser;

use logging::init_cli_logging;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // Start logging in debug if the verbose flag is present.
    init_cli_logging(
        if cli.verbose {LevelFilter::Debug} else {LevelFilter::Info}
    ).expect("Failed to initialize logging.");

    debug!("Logging successfully initialized.");
    #[cfg(debug_assertions)]
    warn!("Debug build. Do not use in production.");
}