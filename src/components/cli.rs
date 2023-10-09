use super::utils::WSLDistro;
use clap::{crate_authors, crate_version, CommandFactory, Parser};
use clap_help::Printer;
use std::process::exit;

static INTRODUCTION: &str = "Install and configure wsl with ease! \n\nmade with ♥ by h4rl";

#[derive(Parser, Debug)]
#[command(
    name = "Windows Toolkit",
    author,
    version,
    about,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Args {
    #[arg(long, short = 'h', short_alias = '?', help = "Prints help information")]
    pub help: bool,

    #[arg(long, short = 'v', help = "Prints version information")]
    pub version: bool,

    #[arg(long, short = 'w', help = "Setup wsl")]
    pub wsl: Option<String>,
}

fn print_help() {
    Printer::new(Args::command())
        .with("introduction", INTRODUCTION)
        .without("author")
        .print_help();
}

pub fn cli_init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.help {
        print_help();
        exit(0);
    }
    if args.version {
        println!(
            "Windows Toolkit v{} by {}",
            crate_version!(),
            crate_authors!()
        );
        exit(0);
    }
    Ok(())
}
