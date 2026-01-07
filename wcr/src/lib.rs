use std::error::Error;
use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'l', long = "lines", help = "Number lines")]
    lines: bool,

    #[arg(short = 'w', long = "words", help = "Number words")]
    words: bool,

    #[arg(short = 'c', long = "bytes", help = "Number bytes")]
    bytes: bool,

    #[arg(short = 'm', long = "chars", help = "Number characters")]
    chars: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("Files: {:#?}", config.files);
    Ok(())
}