use std::process::exit;

use clap::Parser;
use tc::cli::Args;

fn main() {
    let args = Args::parse();

    if let Err(e) = tc::run(args) {
        eprintln!("Error: {e}");
        exit(1);
    }
}
