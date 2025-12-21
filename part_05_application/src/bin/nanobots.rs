use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};

#[derive(Deserialize, Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

#[derive(Deserialize, Debug)]
struct InputData {
    bots: Vec<Bot>,
}

fn get_input() -> Vec<Bot> {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn Read> = if args.len() > 1 {
        Box::new(File::open(&args[1]).expect("Failed to open file"))
    } else {
        Box::new(io::stdin())
    };

    let data: InputData = serde_json::from_reader(reader).expect("Failed to parse JSON input");
    data.bots
}

fn main() {
    let bots = get_input();
    // YOUR CODE HERE
}
