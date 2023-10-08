mod components;
use components::{cli::cli_init, interface::interface_init};
use std::env::args;

/*
* Developed by h4rl with ♥
* uses bsd-2-clause license :)
*/

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() > 1 {
        cli_init().expect("Failed to initialize CLI");
    } else {
        interface_init().expect("Failed to initialize interface");
    }
}
