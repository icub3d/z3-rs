use z3::ast::Int;
use z3::{SatResult, Solver};

/// Example 1: Basic Solver Usage
/// Goal: Find integers x and y such that:
///   1. x > 10
///   2. y > 10
///   3. x + y = 25
fn demonstrate_basic_solving() {
    println!("--- Basic Solving ---");

    // Create a Solver
    let solver = Solver::new();

    // Define Variables
    let x = Int::new_const("x");
    let y = Int::new_const("y");

    // Assert Constraints
    solver.assert(x.gt(Int::from_i64(10)));
    solver.assert(y.gt(Int::from_i64(10)));
    solver.assert((&x + &y).eq(Int::from_i64(25)));

    // Check for Satisfiability and Retrieve Model
    if solver.check() == SatResult::Sat {
        // If SAT, we can get a "Model", which contains concrete values for our variables.
        let model = solver.get_model().unwrap();
        println!("{model:?}");
    } else {
        println!("UNSAT");
    }
}

/// Example 2: Finding Multiple Solutions
/// Goal: Find ALL distinct pairs (x, y) such that:
///   1. 0 <= x, y <= 2
///   2. x + y = 2
///
/// Strategy:
///   1. Find a solution.
///   2. Add a constraint that "blocks" this solution (NOT (x=val AND y=val)).
///   3. Repeat until UNSAT.
fn demonstrate_multiple_solutions() {
    println!("\n--- Finding Multiple Solutions ---");

    // Create a Solver
    let solver = Solver::new();

    // Define Variables
    let x = Int::new_const("x");
    let y = Int::new_const("y");
    let zero = Int::from_i64(0);
    let two = Int::from_i64(2);

    // Assert Constraints
    solver.assert(x.ge(&zero));
    solver.assert(x.le(&two));
    solver.assert(y.ge(&zero));
    solver.assert(y.le(&two));
    solver.assert((&x + &y).eq(&two));

    let mut count = 0;

    // Use the `solutions` iterator to automatically find all satisfying assignments
    // for the tuple (x, y). The second argument `true` enables model completion.
    for (x_sol, y_sol) in solver.solutions((&x, &y), true) {
        count += 1;
        // The iterator returns new AST nodes representing the concrete values from the model
        let x_val = x_sol.as_i64().unwrap();
        let y_val = y_sol.as_i64().unwrap();

        println!("Solution #{}: x = {}, y = {}", count, x_val, y_val);
    }
    println!("Found {} total solutions.", count);
}

/// Example 3: Advent of Code 2023 Day 24 (Part 2)
/// Goal: Find the initial position and velocity of a rock that will collide with
/// all hailstones at some point in time.
///
/// Math:
///   For each hailstone i, there exists a time t_i >= 0 such that:
///     RockPos + RockVel * t_i = HailPos_i + HailVel_i * t_i
///   
///   This gives us 3 equations (x, y, z) for each hailstone.
fn solve_aoc_day24() {
    println!("\n--- Advent of Code 2023 Day 24 (Part 2) ---");

    // Setup our problem.
    struct Hailstone {
        px: i64,
        py: i64,
        pz: i64,
        vx: i64,
        vy: i64,
        vz: i64,
    }

    // A subset of the example input from AoC
    let hailstones = [
        Hailstone {
            px: 19,
            py: 13,
            pz: 30,
            vx: -2,
            vy: 1,
            vz: -2,
        },
        Hailstone {
            px: 18,
            py: 19,
            pz: 22,
            vx: -1,
            vy: -1,
            vz: -2,
        },
        Hailstone {
            px: 20,
            py: 25,
            pz: 34,
            vx: -2,
            vy: -2,
            vz: -4,
        },
        Hailstone {
            px: 12,
            py: 31,
            pz: 28,
            vx: -1,
            vy: -2,
            vz: -1,
        },
        Hailstone {
            px: 20,
            py: 19,
            pz: 15,
            vx: 1,
            vy: -5,
            vz: -3,
        },
    ];

    // Create our solver.
    let solver = Solver::new();

    // Define our variables.
    let rpx = Int::new_const("rpx");
    let rpy = Int::new_const("rpy");
    let rpz = Int::new_const("rpz");
    let rvx = Int::new_const("rvx");
    let rvy = Int::new_const("rvy");
    let rvz = Int::new_const("rvz");

    // Define and Add Constraints for each Hailstone
    for (i, h) in hailstones.iter().enumerate() {
        // Define a unique time variable t_i for this collision
        // Each collision happens at a different time!
        let t = Int::new_const(format!("t_{}", i));

        // Constraint: Collision must happen in the future (t >= 0)
        solver.assert(t.ge(0));

        // Physics Equation: Position = Start + Velocity * Time
        // We assert that Rock's position at time t equals Hailstone's position at time t.

        // X-coordinate collision
        let hpx = Int::from_i64(h.px);
        let hvx = Int::from_i64(h.vx);
        // rpx + rvx * t == hpx + hvx * t
        solver.assert((&rpx + &rvx * &t).eq(&(&hpx + &hvx * &t)));

        // Y-coordinate collision
        let hpy = Int::from_i64(h.py);
        let hvy = Int::from_i64(h.vy);
        solver.assert((&rpy + &rvy * &t).eq(&(&hpy + &hvy * &t)));

        // Z-coordinate collision
        let hpz = Int::from_i64(h.pz);
        let hvz = Int::from_i64(h.vz);
        solver.assert((&rpz + &rvz * &t).eq(&(&hpz + &hvz * &t)));
    }

    println!("Solver checking... (this might take a moment)");

    // Check and Solve
    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();

        // Extract Results
        let x = model.eval(&rpx, true).unwrap().as_i64().unwrap();
        let y = model.eval(&rpy, true).unwrap().as_i64().unwrap();
        let z = model.eval(&rpz, true).unwrap().as_i64().unwrap();
        let vx = model.eval(&rvx, true).unwrap().as_i64().unwrap();
        let vy = model.eval(&rvy, true).unwrap().as_i64().unwrap();
        let vz = model.eval(&rvz, true).unwrap().as_i64().unwrap();

        println!("Found Rock Trajectory!");
        println!("Position: ({}, {}, {})", x, y, z);
        println!("Velocity: ({}, {}, {})", vx, vy, vz);
        println!("Sum of coordinates: {}", x + y + z);
    } else {
        println!("No solution found.");
    }
}

fn main() {
    demonstrate_basic_solving();
    demonstrate_multiple_solutions();
    solve_aoc_day24();
}
