use clap::Parser;
use std::{fs::read_to_string, io};

/// Print a word count for each FILE. A word is a non-zero-length
/// sequence of printable characters delimited by white space. {n}
/// With no FILE, read standard input.
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), about, long_about = None, verbatim_doc_comment)]
pub struct Args {
    #[arg()]
    pub file: Option<Vec<String>>,
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
