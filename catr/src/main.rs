fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
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
