use clap::Parser;
use std::{fs::read_to_string, io};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style},
};

use crate::counters::*;

/// Print newline, word, and byte counts for each FILE, and a total line if
/// more than one FILE is specified.  A word is a non-zero-length sequence of
/// printable characters delimited by white space. {n}
/// With no FILE, or when FILE is -, read standard input. {n}
/// The options below may be used to select which counts are printed, always in
/// the following order: newline, word, character, byte, maximum line length.
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), verbatim_doc_comment)]
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

impl Content {
    fn get_record(&self, args: &Args) -> Vec<String> {
        let mut record = Vec::new();
        let text = &self.text;

        if args.lines {
            record.push(count_lines(text).to_string());
        }

        if args.words {
            record.push(count_words(text).to_string());
        }

        if args.chars {
            record.push(count_chars(text).to_string());
        }

        if args.bytes {
            record.push(count_bytes(text).to_string());
        }

        if args.max_line_length {
            record.push(get_max_line_length(text).to_string());
        }

        record.push(self.source.clone());
        record
    }

    pub fn bulk_display(args: &Args, content: Vec<Content>) {
        let mut builder = Builder::default();

        for (i, item) in content.iter().enumerate() {
            builder.insert_record(i, item.get_record(args));
        }

        if content.len() > 1 {
            let total = Content {
                source: "total".to_string(),
                text: content
                    .iter()
                    .map(|c| c.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
            };
            builder.insert_record(content.len(), total.get_record(args));
        }

        let mut table = builder.build();
        table.with(Style::empty()).with(Alignment::right());
        println!("{table}");
    }
}

fn read_from_stdin() -> io::Result<Content> {
    Ok(Content {
        source: String::new(),
        text: io::read_to_string(io::stdin())?,
    })
}

pub fn get_content(args: &Args) -> io::Result<Vec<Content>> {
    if let Some(files) = &args.file {
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
