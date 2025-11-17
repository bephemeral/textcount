use std::{error::Error, fs};

use crate::{cli::Args, counters::count_words};

pub mod cli;
mod counters;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.file)?;
    let word_count = count_words(contents.as_str());

    println!("{} {}", word_count, args.file);

    Ok(())
}
