use z3::ast::Int;
use z3::{SatResult, Solver};

/// Example 1: Basic Scopes
/// Goal: Show how push/pop isolate constraints.
fn demonstrate_basic_scopes() {
    println!("--- Basic Scopes ---");

    let solver = Solver::new();
    let x = Int::new_const("x");
    let val_10 = Int::from_i64(10);
    let val_5 = Int::from_i64(5);
    let val_2 = Int::from_i64(2);

    // Base Constraint: x < 10
    println!("Asserting (Base): x < 10");
    solver.assert(x.lt(&val_10));

    // Enter Scope 1
    println!(">> Pushing Scope 1");
    solver.push();

    // Add Temporary Constraint: x > 5
    println!("   Asserting (Scope 1): x > 5");
    solver.assert(x.gt(&val_5));

    // Check
    print!("   Checking Scope 1: ");
    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("SAT (x = {})", model.eval(&x, true).unwrap());
    } else {
        println!("UNSAT");
    }

    // Exit Scope 1
    println!("<< Popping Scope 1 (removing x > 5)");
    solver.pop(1);

    // Enter Scope 2
    // Now we are back to just (x < 10).
    // If we hadn't popped, x==2 would conflict with x>5.
    println!(">> Pushing Scope 2");
    solver.push();

    println!("   Asserting (Scope 2): x == 2");
    solver.assert(x.eq(&val_2));

    print!("   Checking Scope 2: ");
    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("SAT (x = {})", model.eval(&x, true).unwrap());
    } else {
        println!("UNSAT");
    }

    println!("<< Popping Scope 2");
    solver.pop(1);
}

fn main() {
    demonstrate_basic_scopes();
}
