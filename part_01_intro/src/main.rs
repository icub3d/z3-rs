use z3::Optimize;
use z3::Solver;
use z3::ast::Int;

/// Example 1: System of Integer Equations
/// Goal: Find integers x and y such that:
///   1. x + y = 10
///   2. x - y = 2
fn solve_system_of_equations() {
    println!("--- System of Integer Equations ---");

    // Create a Solver
    let solver = Solver::new();

    // Define Variables
    let x = Int::new_const("x");
    let y = Int::new_const("y");

    // Assert Constraints
    solver.assert((&x + &y).eq(10));
    solver.assert((&x - &y).eq(2));

    // Check for Satisfiability
    println!("Solving System:");
    println!("  x + y = 10");
    println!("  x - y = 2");

    match solver.check() {
        z3::SatResult::Sat => {
            println!("result: SATISFIABLE");

            // Retrieve the Model
            let model = solver.get_model().unwrap();

            println!("solution:");
            println!("{model:?}");
        }
        z3::SatResult::Unsat => println!("result: UNSATISFIABLE"),
        z3::SatResult::Unknown => println!("result: UNKNOWN"),
    }
    println!();
}

/// Example 2: Optimization
/// Goal: Minimize x + y such that:
///   1. x > 0
///   2. y > 0
///   3. 2x + y >= 10
fn solve_optimization() {
    println!("--- Optimization Example ---");

    // Create a Solver (Optimizer in this case)
    let optimizer = Optimize::new();

    // Define Variables and Constants
    let x = Int::new_const("x");
    let y = Int::new_const("y");

    // Assert Constraints
    optimizer.assert(&x.gt(0));
    optimizer.assert(&y.gt(0));
    optimizer.assert(&(2i64 * &x + &y).ge(10));

    println!("Optimizing system:");
    println!("  Minimize: x + y");
    println!("  Subject to:");
    println!("    x > 0");
    println!("    y > 0");
    println!("    2x + y >= 10");

    // Objective: Minimize x + y
    let sum = &x + &y;
    optimizer.minimize(&sum);

    // Check for Satisfiability
    if optimizer.check(&[]) == z3::SatResult::Sat {
        println!("result: SATISFIABLE");
        // Retrieve the Model
        let model = optimizer.get_model().unwrap();

        // Get variables from the model.
        let x_val = model.eval(&x, true).unwrap().as_i64().unwrap();
        let y_val = model.eval(&y, true).unwrap().as_i64().unwrap();
        let sum_val = model.eval(&sum, true).unwrap().as_i64().unwrap();

        println!("Optimal Solution:");
        println!("  x = {}", x_val);
        println!("  y = {}", y_val);
        println!("  min(x + y) = {}", sum_val);
    } else {
        println!("Result: UNSATISFIABLE");
    }
    println!();
}

fn main() {
    solve_system_of_equations();
    solve_optimization();
}
