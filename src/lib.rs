use crate::cli::{Args, Content, get_content};
use std::error::Error;

pub mod cli;
mod counters;

pub fn run(mut args: Args) -> Result<(), Box<dyn Error>> {
    if !(args.bytes || args.chars || args.lines || args.max_line_length || args.words) {
        args.lines = true;
        args.words = true;
        args.chars = true;
    }

    let content = get_content(&args)?;

    Content::bulk_display(&args, content);

    Ok(())
}
