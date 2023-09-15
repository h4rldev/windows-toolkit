use super::utils::WSLDistro;
use clap::{CommandFactory, Parser};
use clap_help::Printer;
use std::process::exit;

static INTRODUCTION: &str = "Install and configure wsl with ease! \n\nmade with ❤  by h4rl";

#[derive(Parser, Debug)]
#[command(
    name = "wslinstaller",
    author,
    version,
    about,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Args {
    #[arg(long, short = 'h', short_alias = '?', help = "Prints help information")]
    pub help: bool,

    #[clap(help = "Setup the WSL environment")]
    pub setup: Option<String>,

    #[arg(long, short = 'v', help = "Prints version information")]
    pub version: bool,

    #[arg(long, short = 'd', help = "Pick a distro")]
    pub distro: Option<String>,
}

fn print_help() {
    Printer::new(Args::command())
        .with("introduction", INTRODUCTION)
        .without("author")
        .print_help();
}

pub fn cli_init() {
    let args = Args::parse();
    if args.help {
        print_help();
        exit(0);
    }
    if args.version {
        println!(
            "WSL Installer v{}",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );
        exit(0);
    }
    match args.distro {
        Some(value) => {
            let distro: WSLDistro = value.parse().expect("Failed to parse WSLDistro");
            println!("{:?}", distro);
        }
        None => {}
    }
    match args.setup {
        Some(value) => match value.as_str() {
            "setup" => {
                println!("setup");
            }
            _ => {
                println!("setup");
            }
        },
        None => {
            println!("setup");
        }
    }
}
