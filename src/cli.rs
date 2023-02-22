use std::ops::Range;

use clap::{Arg, Command};
use rand::Rng;

pub fn command() -> String {
    let m = Command::new("romazava")
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
        .subcommand(
            Command::new("text")
                .about("Generate a random text")
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .required(false)
                        .help("Length of the text"),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .required(false)
                        .help("File to read the text"),
                )
                .arg(
                    Arg::new("language")
                        .short('L')
                        .long("language")
                        .required(false)
                        .help("Language of the text"),
                ),
        );
    // get the values of the arguments
    let matches = m.get_matches();
    if let Some(values) = matches.subcommand_matches("number") {
        let default_range = String::from("0..10");
        let range = values.get_one::<String>("range").unwrap_or(&default_range);
        let range: Vec<&str> = range.split("..").map(|s| s.trim()).collect();
        let min = range[0].parse::<i8>().unwrap_or(0);
        let max = range[1].parse::<i8>().unwrap_or(10) + 1;
        let mut range = min..max;
        if min > max {
            range = max..min;
        } else if min == max {
            range = min..max + 1;
        }
        return generate_number(range).to_string();
    }
    if let Some(values) = matches.subcommand_matches("boolean") {
        let default_true = String::from("true");
        let default_false = String::from("false");
        let true_value = values
            .get_one::<String>("true_value")
            .unwrap_or(&default_true);
        let false_value = values
            .get_one::<String>("false_value")
            .unwrap_or(&default_false);
        if generate_boolean() {
            return true_value.to_string();
        } else {
            return false_value.to_string();
        }
    }
    return String::from("Hello World");
}

fn generate_number(range: Range<i8>) -> i8 {
    print!("\nGenerate a random number: ");
    rand::thread_rng().gen_range(range)
}

fn generate_boolean() -> bool {
    print!("\nGenerate a random boolean: ");
    rand::thread_rng().gen()
}
