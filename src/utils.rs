use std::ops::Range;
use rand::{Rng};

pub fn generate_number(range: Range<i8>) -> i8 {
  rand::thread_rng().gen_range(range)
}

pub fn generate_boolean() -> bool {
rand::thread_rng().gen()
}