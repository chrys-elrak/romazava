use clap::{Arg, Command};

use crate::romazava::Romazava;

pub fn command(mut romazava: Romazava) -> Romazava {
    let mut m = Command::new("romazava")
        .version("1.0")
        .author("Chrys R. <chrysrakk@gmail.com>")
        .about("Generate random data")
        .subcommand(
            Command::new("number")
                .about("Generate a random number")
                .arg(
                    Arg::new("range")
                        .short('r')
                        .long("range")
                        .help("Range of the number"),
                ),
        )
        .subcommand(
            Command::new("boolean")
                .about("Generate a random boolean")
                .arg(
                    Arg::new("true_value")
                        .short('t')
                        .long("true")
                        .help("True value"),
                )
                .arg(
                    Arg::new("false_value")
                        .short('f')
                        .long("false")
                        .help("False value"),
                ),
        )
        .subcommand(Command::new("emoji").about("Generate a random emoji"))
        .subcommand(
            Command::new("word")
                .about("Generate a random word")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .required(false)
                        .help("File to read the word"),
                )
                .arg(
                    Arg::new("language")
                        .short('L')
                        .long("language")
                        .required(false)
                        .exclusive(true)
                        .help("Language of the word"),
                ),
        );
    let matches = m.clone().get_matches();
    match matches.subcommand() {
        Some(("number", values)) => {
            let default_range = String::from("0..10");
            let range = values.get_one::<String>("range").unwrap_or(&default_range);
            let range: Vec<&str> = range.split("..").map(|s| s.trim()).collect();
            let min = range[0].parse::<i8>().unwrap_or(0);
            let max = range[1].parse::<i8>().unwrap_or(10) + 1;
            romazava.number(min, max);
        }
        Some(("boolean", values)) => {
            let default_true = String::from("true");
            let default_false = String::from("false");
            let true_value = values
                .get_one::<String>("true_value")
                .unwrap_or(&default_true);
            let false_value = values
                .get_one::<String>("false_value")
                .unwrap_or(&default_false);
            romazava.boolean(true_value.to_owned(), false_value.to_owned());
        }
        Some(("emoji", values)) => {
            let x = values.subcommand();
            println!("{:?}", x);
            romazava.emoji();
        }
        Some(("word", values)) => {
            let mut l: String = String::new();
            let mut f: Option<String> = None;
            if let Some(lang) = values.get_one::<String>("language") {
                l = lang.clone();
            }
            if let Some(file) = values.get_one::<String>("file") {
                f = Some(file.clone());
            }
            romazava.word(l.as_str(), f);
        }
        _ => {
            m.print_help().unwrap();
        }
    }
    return romazava;
}
