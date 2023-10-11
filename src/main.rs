//! #Developed by h4rl with ♥
//! #uses bsd-2-clause license :)

mod components;
use check_elevation::is_elevated;
use components::{cli::cli_init, interface::interface_init};
use std::env::args;
use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::fmt;

#[instrument(name = "main", level = "debug")]
fn main() {
    fmt::init();
    if !is_elevated().expect("Failed to check if elevated") {
        warn!("The program isn't elevated, some issues may occur!");
    }

    let args = &args().collect::<Vec<String>>();
    println!("{:?}", args);
    if args.len() > 1 {
        cli_init().expect("Failed to initialize CLI");
    } else {
        interface_init().expect("Failed to initialize interface");
    }
}
