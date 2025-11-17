use clap::Parser;
use std::{fs::read_to_string, io};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
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
