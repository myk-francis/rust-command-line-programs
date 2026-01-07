use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short, long)]
    number_lines: bool,

    /// Number non-empty lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}


type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}