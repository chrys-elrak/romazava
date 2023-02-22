use std::ops::Range;

use clap::{arg, command, Parser, Subcommand};
use rand::Rng;

#[derive(Subcommand, Debug)]
enum ResultType {
    Number,
    Text,
    Boolean,
}
struct ResultValue {
    _number: Option<i8>,
    _text: Option<String>,
    _boolean: Option<bool>,
}

impl ResultValue {
    pub fn from_number(n: i8) -> Self {
        Self {
            _number: Some(n),
            _text: None,
            _boolean: None,
        }
    }
    pub fn from_bool(n: bool) -> Self {
        Self {
            _boolean: Some(n),
            _text: None,
            _number: None,
        }
    }
    pub fn from_text(n: String) -> Self {
        Self {
            _text: Some(n),
            _number: None,
            _boolean: None,
        }
    }
    pub fn get_value(self) -> String {
        if self._boolean != None {
            let x = if self._boolean.unwrap() == true {
                String::from("Yeah")
            } else {
                String::from("No")
            };
            return x;
        };
        if self._number != None {
            return self._number.unwrap().to_string();
        };
        if self._text != None {
            return self._text.unwrap().to_string();
        };
        String::from("No value")
    }
}
trait RandomGetter {
    fn get_random_number(self, range: Range<i8>) -> i8;
    fn get_random_boolean(self) -> bool;
    fn get_random_text(self) -> String;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File to read from, if not provided use current directory and find the first file
    #[arg(short, long, default_value = None)]
    content_provider: Option<String>,
    /// Type of the output => number, string, boolean
    /// Default: number
    #[command(subcommand)]
    result_type: Option<ResultType>,
}

impl Args {
    pub fn get_random_value(&self) -> String {
        let data: Data = Data::new();
        let x: ResultValue;
        match self.result_type {
            Some(ResultType::Number) => {
                x = ResultValue::from_number(data.get_random_number(0..10));
            }
            Some(ResultType::Boolean) => {
                x = ResultValue::from_bool(data.get_random_boolean());
            }
            Some(ResultType::Text) => {
                x = ResultValue::from_text(data.get_random_text());
            }
            _ => panic!("No type provided"),
        }
        x.get_value()
    }
}

pub struct Data {
    pub source: String,
    pub value: Option<String>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            value: None,
        }
    }
}

impl RandomGetter for Data {
    fn get_random_number(self, range: Range<i8>) -> i8 {
        rand::thread_rng().gen_range(range)
    }

    fn get_random_boolean(self) -> bool {
        rand::random::<bool>()
    }

    fn get_random_text(self) -> String {
        todo!()
    }
}
