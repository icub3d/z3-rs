use z3::ast::Int;
use z3::Solver;

fn main() {
    println!("--- Homework: Bakery Receipt ---");

    let solver = Solver::new();

    // Variables
    let c = Int::new_const("c"); // Croissants
    let b = Int::new_const("b"); // Bagels
    let m = Int::new_const("m"); // Muffins

    let _zero = Int::from_i64(0);
    let one = Int::from_i64(1);
    
    // Constraints:
    // 1. At least one of each
    solver.assert(c.ge(&one));
    solver.assert(b.ge(&one));
    solver.assert(m.ge(&one));

    // 2. More Bagels than Muffins
    solver.assert(b.gt(&m));

    // 3. Total Items = 20
    let count = &c + &b + &m;
    solver.assert(count.eq(Int::from_i64(20)));

    // 4. Total Cost = $50.00
    // 3*c + 2*b + 4*m = 50
    let cost = &c * 3 + &b * 2 + &m * 4;
    solver.assert(cost.eq(Int::from_i64(50)));

    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("Solution found:");
        println!("  Croissants: {}", model.eval(&c, true).unwrap());
        println!("  Bagels:     {}", model.eval(&b, true).unwrap());
        println!("  Muffins:    {}", model.eval(&m, true).unwrap());
    } else {
        println!("No solution found.");
    }
}
