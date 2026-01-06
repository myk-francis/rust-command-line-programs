use clap::{Arg, Command};

pub fn build_sentence(values: impl Iterator<Item = String>) -> String {
    values.collect::<Vec<_>>().join(" ")
}

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("myk")
        .about("Echo with more features")
        .arg(
            Arg::new("message")
                .short('m')
                .required(true)
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("no_newline")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(values) = matches.get_many::<String>("message") {
        let sentence = build_sentence(values.cloned());

        if matches.get_flag("omit_newline") {
            print!("{}", sentence);
        } else {
            println!("{}", sentence);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_words_with_spaces() {
        let input = vec![
            "hello".to_string(),
            "world".to_string(),
            "from".to_string(),
            "rust".to_string(),
        ];

        let result = build_sentence(input.into_iter());

        assert_eq!(result, "hello world from rust");
    }

    #[test]
    fn single_word_is_unchanged() {
        let input = vec!["hello".to_string()];

        let result = build_sentence(input.into_iter());

        assert_eq!(result, "hello");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let input: Vec<String> = vec![];

        let result = build_sentence(input.into_iter());

        assert_eq!(result, "");
    }
}
