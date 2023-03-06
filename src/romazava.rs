use std::fmt::{Display, Formatter};

use rand::Rng;
use serde_json::Value;

use crate::utils::{generate_boolean, generate_number};

pub struct Romazava {
    operation: String,
    result: String,
}

impl Romazava {
    pub fn new() -> Self {
        Self {
            operation: String::new(),
            result: String::new(),
        }
    }
    pub fn is_ok(&self) -> bool {
        !self.operation.is_empty()
    }
    pub fn number(&mut self, min: i8, max: i8) {
        self.operation = format!("Generate a random number between {} and {}", min, max - 1);
        let mut range = min..max;
        if min > max {
            range = max..min;
        } else if min == max {
            range = min..max + 1;
        }
        self.result = generate_number(range).to_string();
    }

    pub fn boolean(&mut self, true_value: String, false_value: String) {
        self.operation = format!("Generate a random boolean");
        if generate_boolean() {
            self.result = true_value;
        }
        self.result = false_value;
    }

    pub fn emoji(&mut self) {
        self.operation = format!("Generate a random emoji");
        let file = std::fs::read_to_string("emoji.json").unwrap();
        let emoji: Value = serde_json::from_str(&file).unwrap();
        let _arr = emoji.as_array().unwrap();
        let random_index = rand::thread_rng().gen_range(0.._arr.len());
        let emoji = (_arr[random_index]).as_object().unwrap().clone();
        self.result = serde_json::to_string_pretty(&emoji).unwrap();
    }

    pub fn word(&mut self, _lang: &str, file: Option<String>) {
        self.operation = format!("Generate a random word");
        // Todo: Add more languages
        let filename: &str = match _lang {
            "mg" => "teny.json",
            "fr" => "mots.json",
            _ => "words.json",
        };
        // Todo: Configurable word length
        // check file type is json
        let file = std::fs::read_to_string(file.unwrap_or(filename.to_string()));
        if let Ok(f) = file {
            let words = f.split_whitespace().collect::<Vec<&str>>();
            let random_index = rand::thread_rng().gen_range(0..words.len());
            self.result = words[random_index].to_string();
        } else {
            let words: Vec<Value> = serde_json::from_str(&file.unwrap()).unwrap();
            let random_index = rand::thread_rng().gen_range(0..words.len());
            self.result = words[random_index].to_string();
        }
    }
}

impl Display for Romazava {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation: {}\nResult: {}", self.operation, self.result)
    }
}
