use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use z3::ast::Int;
use z3::{Optimize, SatResult};

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

// Helper function to get the distance from a to b as an Int (via ge/ite).
fn distance(a: &Int, b: i64) -> Int {
    let b_val = Int::from_i64(b);
    let diff = a - &b_val;
    let neg_diff = &b_val - a;
    a.ge(&b_val).ite(&diff, &neg_diff)
}

fn main() {
    let bots = get_input();
    let opt = Optimize::new();

    // Target Coordinates
    let tx = Int::new_const("tx");
    let ty = Int::new_const("ty");
    let tz = Int::new_const("tz");

    // Maximize In-Range Bots
    for bot in &bots {
        // Dist = |tx - bx| + |ty - by| + |tz - bz|
        let d = &distance(&tx, bot.x) + &distance(&ty, bot.y) + &distance(&tz, bot.z);

        // In Range: Dist <= r
        let in_range = d.le(Int::from_i64(bot.r));

        // Soft constraint: prefer satisfying this (weight 1)
        opt.assert_soft(&in_range, 1, None);
    }

    // Secondary Objective: Minimize Distance to Origin
    let dist_origin = &distance(&tx, 0) + &distance(&ty, 0) + &distance(&tz, 0);
    opt.minimize(&dist_origin);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let x = model.eval(&tx, true).unwrap();
        let y = model.eval(&ty, true).unwrap();
        let z = model.eval(&tz, true).unwrap();

        println!("Optimal Coordinate: ({}, {}, {})", x, y, z);
        println!(
            "Solution: {}",
            x.as_i64().unwrap().abs() + y.as_i64().unwrap().abs() + z.as_i64().unwrap().abs()
        );
    } else {
        println!("UNSAT");
    }
}
