mod components;
use components::{cli::cli_init, interface::interface_init, utils::is_elevated};
use std::env::args;

/*
* Developed by h4rl with ♥
* uses bsd-2-clause license :)
*/

fn main() {
    let elevation_status = is_elevated().expect("Failed to check if elevated");
    println!("Elevation Status: {}", elevation_status);

    let args = args().collect::<Vec<String>>();
    println!("{:?}", args);
    if args.len() > 1 {
        cli_init().expect("Failed to initialize CLI");
    } else {
        interface_init().expect("Failed to initialize interface");
    }
}
