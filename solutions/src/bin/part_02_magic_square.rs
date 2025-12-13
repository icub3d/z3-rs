use z3::ast::{Ast, Int};
use z3::Solver;

fn main() {
    println!("--- Homework: Magic Square ---");

    let solver = Solver::new();

    // Create 3x3 Grid
    let mut grid = Vec::new();
    for r in 0..3 {
        let mut row = Vec::new();
        for c in 0..3 {
            row.push(Int::new_const(format!("x_{}_{}", r, c).as_str()));
        }
        grid.push(row);
    }

    let one = Int::from_i64(1);
    let nine = Int::from_i64(9);
    let target = Int::from_i64(15);

    // Collect all cells for distinct constraint
    let mut all_cells = Vec::new();

    // 1. Range Constraints (1..9)
    for row in &grid {
        for cell in row {
            solver.assert(cell.ge(&one));
            solver.assert(cell.le(&nine));
            all_cells.push(cell);
        }
    }

    // 2. Distinct Values
    solver.assert(<Int as Ast>::distinct(&all_cells));

    // 3. Row Sums
    for row in &grid {
        let sum = &row[0] + &row[1] + &row[2];
        solver.assert(sum.eq(&target));
    }

    // 4. Col Sums
    (0..3).for_each(|c| {
        let sum = &grid[0][c] + &grid[1][c] + &grid[2][c];
        solver.assert(sum.eq(&target));
    });

    // 5. Diagonals
    // TL -> BR
    solver.assert((&grid[0][0] + &grid[1][1] + &grid[2][2]).eq(&target));
    // TR -> BL
    solver.assert((&grid[0][2] + &grid[1][1] + &grid[2][0]).eq(&target));

    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("Magic Square Found:");
        for row in &grid {
            for cell in row {
                print!("{} ", model.eval(cell, true).unwrap());
            }
            println!();
        }
    } else {
        println!("UNSAT");
    }
}
