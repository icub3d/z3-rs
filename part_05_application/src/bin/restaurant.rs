use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};

#[derive(Deserialize, Debug)]
struct InputData {
    budget: i64,
    restaurants: Vec<Restaurant>,
    people: Vec<Person>,
}

#[derive(Deserialize, Debug)]
struct Restaurant {
    name: String,
    cost: i64,
    vegan: bool,
}

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    is_vegan: bool,
    ratings: Vec<i64>, // rating per restaurant index
}

fn get_input() -> InputData {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn Read> = if args.len() > 1 {
        Box::new(File::open(&args[1]).expect("Failed to open file"))
    } else {
        Box::new(io::stdin())
    };

    serde_json::from_reader(reader).expect("Failed to parse JSON input")
}

fn main() {
    let data = get_input();
    // YOUR CODE HERE
}
