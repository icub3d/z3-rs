use std::env;
use std::fs::File;
use std::io::{self, Read};
use z3::{Optimize, SatResult};
use z3::ast::{Ast, Int};
use serde::Deserialize;

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

    println!("Loaded {} nanobots.", bots.len());

    let opt = Optimize::new();

    // 1. Define Variables (tx, ty, tz)
    
    // 2. Define Metric (Manhattan Distance)
    
    // 3. Maximize Count of bots in range
    //    bot[i] is in range if |tx-bx| + |ty-by| + |tz-bz| <= br

    // 4. Minimize Distance to Origin (Tie-Breaker)

    println!("TODO: Implement optimizer logic!");

    /*
    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        // Print tx, ty, tz and the count
    }
    */
}
