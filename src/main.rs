use reqwest::{self};
use serde::{Deserialize, Serialize};
use std::env;
use std::io;
use dotenv::dotenv;
use colored::Colorize;


#[derive(Deserialize, Serialize, Debug)]
struct Data {
    _id: i32,
    word: String,
}

fn get_word(api_key: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:1000/Random?api_key={}", api_key);
    let response = reqwest::blocking::get(&url)?;

    let word = response.json::<Data>()?;
    Ok(word)
}

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set as environment variable");

    loop {
        println!("Welcome to Rustle!");
        println!("Start new game? (y/n): ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "y" => {
                match get_word(&api_key) {
                    Ok(words) => {
                        let word = words.word;
                        println!("{}", word);

                        let mut guess = String::new();
                        
                        for _ in 1..=5  {
                            guess.clear();
                            io::stdin()
                                .read_line(&mut guess)
                                .expect("Failed to read input");

                            let guess = guess.trim();

                            if guess.len() != word.len() {
                                println!("Enter a word of length 5");
                                continue;
                            }

                            let mut correct = true;
                            for (w, g) in word.chars().zip(guess.chars()) {
                                if w != g {
                                    print!("{}", g.to_string().red());
                                    correct = false;
                                }
                            }

                            if correct {
                                println!("Correct!");
                                break; 
                            } else {
                                println!("Incorrect. Try again.");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching word: {}", e);
                    }
                }
                break;
            }
            "n" => {
                println!("Goodbye!");
                return;
            }
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
            }
        }
    }
}