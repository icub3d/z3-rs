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

fn main() {
    let bots = get_input();
    let opt = Optimize::new();

    // 1. Target Coordinates
    let tx = Int::new_const("tx");
    let ty = Int::new_const("ty");
    let tz = Int::new_const("tz");

    // Helper: Absolute Value
    // abs(a - b) = If(a > b, a - b, b - a)
    fn dist_1d(a: &Int, b: i64) -> Int {
        let b_val = Int::from_i64(b);
        let diff = a - &b_val;
        let neg_diff = &b_val - a;
        a.ge(&b_val).ite(&diff, &neg_diff)
    }

    // 2. Count In-Range Bots
    let mut count = Int::from_i64(0);
    let one = Int::from_i64(1);
    let zero = Int::from_i64(0);

    for bot in &bots {
        // Dist = |tx - bx| + |ty - by| + |tz - bz|
        let d = &dist_1d(&tx, bot.x) + &dist_1d(&ty, bot.y) + &dist_1d(&tz, bot.z);

        // In Range: Dist <= r
        let in_range = d.le(Int::from_i64(bot.r));

        // Add 1 if true, 0 if false
        let val = in_range.ite(&one, &zero);
        count = &count + &val;
    }

    // 3. Primary Objective: Maximize Count
    opt.maximize(&count);

    // 4. Secondary Objective: Minimize Distance to Origin
    let dist_origin = &dist_1d(&tx, 0) + &dist_1d(&ty, 0) + &dist_1d(&tz, 0);
    opt.minimize(&dist_origin);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let x = model.eval(&tx, true).unwrap();
        let y = model.eval(&ty, true).unwrap();
        let z = model.eval(&tz, true).unwrap();
        let c = model.eval(&count, true).unwrap();

        println!("Optimal Coordinate: ({}, {}, {})", x, y, z);
        println!("Bots in range: {}", c);
    } else {
        println!("UNSAT");
    }
}
