use z3::ast::Int;
use z3::{SatResult, Solver};

/// Example 1: Basic Scopes with Dependencies
/// Goal: Show how push/pop isolate constraints while preserving the base AST.
fn demonstrate_basic_scopes() {
    println!("--- Basic Scopes & AST Management ---");

    let solver = Solver::new();
    let x = Int::new_const("x");
    let y = Int::new_const("y");
    let z = Int::new_const("z");

    // 1. Establish the "Base" System
    //    We define a relationship: z = x + y
    //    And a rule: inputs must be positive.
    //    This part of the "AST" (Abstract Syntax Tree) is permanent.
    println!("1. Base Layer:");
    println!("   Asserting: z == x + y");
    solver.assert(z.eq(&(&x + &y)));

    println!("   Asserting: x > 0, y > 0");
    solver.assert(x.gt(0));
    solver.assert(y.gt(0));

    println!("\n{solver:?}\n");

    // 2. Scope 1: Fix the output 'z'
    println!("2. Pushing Scope 1...");
    solver.push();

    println!("   Asserting (Scope 1): z == 10");
    solver.assert(z.eq(10));

    // Check
    print!("   Checking Scope 1: ");
    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        // The solver finds valid x and y that sum to 10
        println!("SAT");
        println!(
            "   Solution: x = {}, y = {}, z = {}",
            model.eval(&x, true).unwrap(),
            model.eval(&y, true).unwrap(),
            model.eval(&z, true).unwrap()
        );
    } else {
        println!("UNSAT");
    }

    println!("\n{solver:?}\n");

    println!("   Popping Scope 1 (removing z == 10). Back to Base.\n");
    solver.pop(1);

    // 3. Scope 2: Fix the inputs 'x' and 'y'
    println!("3. Pushing Scope 2...");
    solver.push();

    println!("   Asserting (Scope 2): x == 20, y == 30");
    solver.assert(x.eq(20));
    solver.assert(y.eq(30));

    // Check
    print!("   Checking Scope 2: ");
    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        // The solver infers z must be 50 because of the base constraint
        println!("SAT");
        println!(
            "   Solution: x = {}, y = {}, z = {}",
            model.eval(&x, true).unwrap(),
            model.eval(&y, true).unwrap(),
            model.eval(&z, true).unwrap()
        );
    } else {
        println!("UNSAT");
    }

    println!("\n{solver:?}\n");

    println!("   Popping Scope 2. Back to Base.\n");
    solver.pop(1);

    // 4. Scope 3: Impossible constraints
    println!("4. Pushing Scope 3...");
    solver.push();

    // x > 0, y > 0 (Base) implies z >= 2 (integers).
    // If we say z == 1, it might be UNSAT if x,y must be distinct or specific,
    // but here x=0 is not allowed (x>0), so min x=1, min y=1 => min z=2.
    // Let's try something clearly conflicting with base + new.

    println!("   Asserting (Scope 3): z == 5, x > 10");
    // If x > 10, then x >= 11. Since y > 0 (y >= 1), then z = x+y >= 12.
    // But we assert z == 5. Contradiction.
    solver.assert(z.eq(5));
    solver.assert(x.gt(10));

    print!("   Checking Scope 3: ");
    if solver.check() == SatResult::Sat {
        println!("SAT");
    } else {
        println!("UNSAT (Contradiction: x>10 implies z>11, but z==5)");
    }

    println!("\n{solver:?}\n");

    println!("   Popping Scope 3.\n");
    solver.pop(1);
}

fn main() {
    demonstrate_basic_scopes();
}

