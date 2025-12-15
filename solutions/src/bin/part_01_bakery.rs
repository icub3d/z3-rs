use z3::ast::Int;
use z3::Solver;

fn main() {
    println!("--- Homework: Bakery Receipt ---");

    let solver = Solver::new();

    // Variables
    let c = Int::new_const("c"); // Croissants
    let b = Int::new_const("b"); // Bagels
    let m = Int::new_const("m"); // Muffins

    // Constraints:
    // 1. At least one of each
    solver.assert(c.ge(1));
    solver.assert(b.ge(1));
    solver.assert(m.ge(1));

    // 2. More Bagels than Muffins
    solver.assert(b.gt(&m));

    // 3. Total Items = 20
    let count = &c + &b + &m;
    solver.assert(count.eq(20));

    // 4. Total Cost = $50.00
    // 3*c + 2*b + 4*m = 50
    let cost = &c * 3 + &b * 2 + &m * 4;
    solver.assert(cost.eq(50));

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
