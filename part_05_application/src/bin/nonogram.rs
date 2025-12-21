use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};

#[derive(Deserialize, Debug)]
struct NonogramPuzzle {
    rows: usize,
    cols: usize,
    row_clues: Vec<Vec<usize>>,
    col_clues: Vec<Vec<usize>>,
}

fn get_input() -> NonogramPuzzle {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn Read> = if args.len() > 1 {
        Box::new(File::open(&args[1]).expect("Failed to open file"))
    } else {
        Box::new(io::stdin())
    };

    serde_json::from_reader(reader).expect("Failed to parse JSON input")
}

fn main() {
    let puzzle = get_input();
    // YOUR CODE HERE
}
