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
    #[arg(short, long)]
    number_lines: bool,

    /// Number non-empty lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}


type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut line_number = 1;
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_number, line);
                        line_number += 1;
                    } else if config.number_nonblank_lines {
                        if !line.trim().is_empty() {
                            println!("{:6}\t{}", line_number, line);
                            line_number += 1;
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            },
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => {
            let file = File::open(filename)?; // ? handles errors immediately
            Ok(Box::new(BufReader::new(file)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_values() {
        // "catr" is index 0 (the executable name)
        let config = Config::try_parse_from(["catr"]).unwrap();
        assert_eq!(config.files, vec!["-"]);
        assert!(!config.number_lines);
    }

    #[test]
    fn test_flags() {
        let config = Config::try_parse_from(["catr", "-n", "-b", "file1.txt"]).unwrap();
        assert!(config.number_lines);
        assert!(config.number_nonblank_lines);
        assert_eq!(config.files, vec!["file1.txt"]);
    }
}