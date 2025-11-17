use crate::{
    cli::{Args, get_content},
    counters::count_words,
};
use std::error::Error;

pub mod cli;
mod counters;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let content = get_content(args)?;

    for item in content {
        println!("{} {}", count_words(&item.text), item.source);
    }

    Ok(())
}
