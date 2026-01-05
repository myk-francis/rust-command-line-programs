use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("myk")
        .about("Echo with more features")
        .arg(
            Arg::new("message")
                .short('m')
                .help("Input string to echor")
                .required(true)
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("no_newline")
                .help("Do not output a newline at the end")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(values) = matches.get_many::<String>("message") {
        let sentence = values
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        if matches.get_flag("omit_newline") {
            print!("{}", sentence);
        } else {
            println!("{}", sentence);
        }
    }
}
