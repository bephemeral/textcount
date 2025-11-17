use clap::Parser;
use std::{fs::read_to_string, io};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
pub struct Args {
    #[arg()]
    pub file: Option<String>,
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

pub fn get_content(args: Args) -> io::Result<Content> {
    if let Some(file) = args.file {
        Ok(Content {
            source: file.clone(),
            text: read_to_string(file)?,
        })
    } else {
        read_from_stdin()
    }
}
