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

    // Collect all cells for distinct constraint
    let mut all_cells = Vec::new();

    // 1. Range Constraints (1..9)
    for row in &grid {
        for cell in row {
            solver.assert(cell.ge(1));
            solver.assert(cell.le(9));
            all_cells.push(cell);
        }
    }

    // 2. Distinct Values
    solver.assert(<Int as Ast>::distinct(&all_cells));

    // 3. Row Sums
    for row in &grid {
        let sum = &row[0] + &row[1] + &row[2];
        solver.assert(sum.eq(15));
    }

    // 4. Col Sums
    (0..3).for_each(|c| {
        let sum = &grid[0][c] + &grid[1][c] + &grid[2][c];
        solver.assert(sum.eq(15));
    });

    // 5. Diagonals
    // TL -> BR
    solver.assert((&grid[0][0] + &grid[1][1] + &grid[2][2]).eq(15));
    // TR -> BL
    solver.assert((&grid[0][2] + &grid[1][1] + &grid[2][0]).eq(15));

    for grid in solver.solutions(&grid, true) {
        println!("Magic Square Found:");
        for row in &grid {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
        println!();
    }
}
