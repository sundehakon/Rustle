use reqwest::{self};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;


#[derive(Deserialize, Serialize, Debug)]
struct Word {
    _id: i32,
    word: String,
}

fn get_word(api_key: &str) -> Result<Word, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:1000/Random?api_key={}", api_key);
    let response = reqwest::blocking::get(&url)?;

    let word = response.json::<Word>()?;
    Ok(word)
}

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set as environment variable");

    match get_word(&api_key) {
        Ok(words) => {
            println!("Received word: {:?}", words);
        }
        Err(e) => {
            eprintln!("Error fetching word: {}", e);
        }
    }
}