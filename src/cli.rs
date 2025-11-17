use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    pub file: String,
}
