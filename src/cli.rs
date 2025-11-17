use clap::Parser;
use std::{fs::read_to_string, io};

/// Print newline, word, and byte counts for each FILE, and a total line if
/// more than one FILE is specified.  A word is a non-zero-length sequence of
/// printable characters delimited by white space. {n}
/// With no FILE, or when FILE is -, read standard input. {n}
/// The options below may be used to select which counts are printed, always in
/// the following order: newline, word, character, byte, maximum line length.
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), about, long_about = None, verbatim_doc_comment)]
pub struct Args {
    #[arg()]
    pub file: Option<Vec<String>>,

    /// print the byte counts
    #[arg(short = 'c', long)]
    pub bytes: bool,

    /// print the character counts
    #[arg(short = 'm', long)]
    pub chars: bool,

    /// print the newline counts
    #[arg(short, long)]
    pub lines: bool,

    /// print the maximum display width
    #[arg(short = 'L', long)]
    pub max_line_length: bool,

    /// print the word counts
    #[arg(short, long)]
    pub words: bool,
}

pub struct Content {
    pub source: String,
    pub text: String,
}

fn read_from_stdin() -> io::Result<Content> {
    Ok(Content {
        source: String::new(),
        text: io::read_to_string(io::stdin())?,
    })
}

pub fn get_content(args: Args) -> io::Result<Vec<Content>> {
    if let Some(files) = args.file {
        files
            .iter()
            .map(|file| {
                let text = read_to_string(file)?;
                Ok(Content {
                    source: file.to_string(),
                    text: text,
                })
            })
            .collect()
    } else {
        Ok(vec![read_from_stdin()?])
    }
}
