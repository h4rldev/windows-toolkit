mod components;
use components::{cli::cli_init, interface::dummy};

fn main() {
    cli_init();
    dummy();
}
