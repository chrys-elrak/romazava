use clap::builder::Str;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    pub multicode: bool,
    pub name: String,
    pub emoji: String,
}

impl Emoji {
    pub fn get_random_emoji(emoji: &Vec<Emoji>) -> &Emoji  {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..emoji.len());
        &emoji[random_index]
    }
}
