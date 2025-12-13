use z3::ast::Int;
use z3::{Optimize, SatResult};

fn main() {
    println!("--- Homework: Production Planning ---");

    let opt = Optimize::new();

    // Variables: Number of units to produce
    let chairs = Int::new_const("chairs");
    let tables = Int::new_const("tables");

    let zero = Int::from_i64(0);

    // 1. Non-negative production
    opt.assert(&chairs.ge(&zero));
    opt.assert(&tables.ge(&zero));

    // 2. Resource Constraints
    // Carpentry: 1h/chair + 4h/table <= 40
    let carpentry_hours = &chairs * 1 + &tables * 4;
    opt.assert(&carpentry_hours.le(Int::from_i64(40)));

    // Painting: 3h/chair + 1h/table <= 40
    let painting_hours = &chairs * 3 + &tables * 1;
    opt.assert(&painting_hours.le(Int::from_i64(40)));

    // 3. Objective: Maximize Profit
    // Profit = $20/chair + $50/table
    let profit = &chairs * 20 + &tables * 50;
    opt.maximize(&profit);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let c_val = model.eval(&chairs, true).unwrap();
        let t_val = model.eval(&tables, true).unwrap();
        let p_val = model.eval(&profit, true).unwrap();

        println!("Optimal Production Plan:");
        println!("  Chairs: {}", c_val);
        println!("  Tables: {}", t_val);
        println!("  Max Profit: ${}", p_val);
    } else {
        println!("UNSAT");
    }
}
