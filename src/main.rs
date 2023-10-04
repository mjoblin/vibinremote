use std::io::Write;
use std::process;

use chrono::Local;
use clap::Parser;
use env_logger;
use log::{error};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Control the Vibin music streamer server with keyboard key presses.",
    long_about = None)
]
pub struct AppArgs {
    /// Configuration filename (JSON)
    #[arg(short, long, value_name = "JSON File")]
    config: String,
}

fn main() {
    // Configure logger
    env_logger::Builder::new()
        .format(|buf, record| {
            // Use local timestamps rather than the default UTC
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();

    // Get configuration file from command line and invoke the main application run()
    let args = AppArgs::parse();

    if let Err(err) = vibinremote::run(&args.config) {
        error!("{err}");
        process::exit(1);
    }
}
