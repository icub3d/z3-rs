use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::ops::{BitAnd, BitOr};
use z3::ast::{Ast, Bool, Int};
use z3::Solver;

#[derive(Deserialize, Debug)]
struct NonogramPuzzle {
    rows: usize,
    cols: usize,
    row_clues: Vec<Vec<usize>>,
    col_clues: Vec<Vec<usize>>,
}

fn get_input() -> NonogramPuzzle {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn Read> = if args.len() > 1 {
        Box::new(File::open(&args[1]).expect("Failed to open file"))
    } else {
        Box::new(io::stdin())
    };

    serde_json::from_reader(reader).expect("Failed to parse JSON input")
}

fn main() {
    let puzzle = get_input();
    let solver = Solver::new();

    // Grid Variables: grid[r][c] -> Bool
    let grid = (0..puzzle.rows)
        .map(|r| {
            (0..puzzle.cols)
                .map(|c| Bool::new_const(format!("g_{}_{}", r, c).as_str()))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    // Add constraints for each row.
    for (r, clues) in puzzle.row_clues.iter().enumerate() {
        let line: Vec<&Bool> = grid[r].iter().collect();
        constrain_line(&solver, &line, clues, format!("r{}", r).as_str());
    }

    // Add constraints for each column.
    for (c, clues) in puzzle.col_clues.iter().enumerate() {
        let line: Vec<&Bool> = (0..puzzle.rows).map(|r| &grid[r][c]).collect();
        constrain_line(&solver, &line, clues, format!("c{}", c).as_str());
    }

    println!("{solver:?}");

    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("Solution:");
        for row in &grid {
            for cell in row {
                let val = model.eval(cell, true).unwrap().as_bool().unwrap();
                print!("{}", if val { "#" } else { " " });
            }
            println!();
        }
    } else {
        println!("Unsolvable.");
    }
}

fn constrain_line(solver: &Solver, line: &[&Bool], clues: &[usize], prefix: &str) {
    let cells_len = line.len() as i64;
    let clues_len = clues.len();

    // Create start position variables for each block
    let starts: Vec<Int> = (0..clues_len)
        .map(|i| Int::new_const(format!("{}_s_{}", prefix, i).as_str()))
        .collect();

    // Constrain start positions for each block
    for (i, (start, &block_len)) in starts.iter().zip(clues.iter()).enumerate() {
        let len = block_len as i64;

        solver.assert(start.ge(0));
        solver.assert((start + len).le(cells_len));

        // Ensure blocks don't overlap (at least 1 gap between blocks)
        if i > 0 {
            let prev_len = clues[i - 1] as i64;
            solver.assert(start.ge(&(&starts[i - 1] + prev_len + 1)));
        }
    }

    // If there are no clues, we are done.
    if clues.is_empty() {
        // Empty line: all cells must be false
        for cell in line {
            solver.assert(Ast::eq(*cell, Bool::from_bool(false)));
        }
        return;
    }

    // Otherwise, constrain each cell. Each cell is within a range if it's between the start and the start's length. We or together all of the starts.
    for (j, cell) in line.iter().enumerate() {
        let j_int = Int::from_i64(j as i64);
        let cell_constraint = starts
            .iter()
            .zip(clues.iter())
            .map(|(start, &block_len)| {
                let len = block_len as i64;
                start.le(&j_int).bitand(&j_int.lt(&(start + len)))
            })
            .reduce(|acc, cond| acc.bitor(&cond))
            .unwrap();

        solver.assert(Ast::eq(*cell, &cell_constraint));
    }
}
