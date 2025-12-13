use std::env;
use std::fs::File;
use std::io::{self, Read};
use z3::Solver;
use z3::ast::{Ast, Bool, Int};
use serde::Deserialize;

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

    // 1. Grid Variables: grid[r][c] -> Bool
    let mut grid: Vec<Vec<Bool>> = Vec::new();
    for r in 0..puzzle.rows {
        let mut row = Vec::new();
        for c in 0..puzzle.cols {
            row.push(Bool::new_const(format!("g_{}_{}", r, c).as_str()));
        }
        grid.push(row);
    }

    for (r, clues) in puzzle.row_clues.iter().enumerate() {
        let line: Vec<&Bool> = grid[r].iter().collect();
        constrain_line(&solver, &line, clues, format!("r{}", r).as_str());
    }

    for (c, clues) in puzzle.col_clues.iter().enumerate() {
        let line: Vec<&Bool> = (0..puzzle.rows).map(|r| &grid[r][c]).collect();
        constrain_line(&solver, &line, clues, format!("c{}", c).as_str());
    }

    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("Solution:");
        println!("  {}", "-".repeat(puzzle.cols * 2 + 2));
        for row in &grid {
            print!("| ");
            for cell in row {
                let val = model.eval(cell, true).unwrap().as_bool().unwrap();
                print!("{}", if val { "##" } else { "  " });
            }
            println!(" |");
        }
        println!("  {}", "-".repeat(puzzle.cols * 2 + 2));
    } else {
        println!("Unsolvable.");
    }
}

fn constrain_line(solver: &Solver, line: &[&Bool], clues: &[usize], prefix: &str) {
    let n_cells = line.len() as i64;
    let n_clues = clues.len();
    
    // Start positions for each block
    let mut starts = Vec::new();
    for i in 0..n_clues {
        starts.push(Int::new_const(format!("{}_s_{}", prefix, i).as_str()));
    }

    // Constraints on Starts
    for i in 0..n_clues {
        let s = &starts[i];
        let len = clues[i] as i64;
        
        // Bounds: s >= 0
        solver.assert(s.ge(Int::from_i64(0)));
        
        // Must fit: s + len <= n_cells
        solver.assert((s + Int::from_i64(len)).le(Int::from_i64(n_cells)));
        
        // Order: s_{i+1} > s_i + len_i (at least 1 gap)
        if i > 0 {
            let prev_s = &starts[i-1];
            let prev_len = clues[i-1] as i64;
            // s_i >= s_{i-1} + len + 1
            solver.assert(s.ge(&(prev_s + Int::from_i64(prev_len + 1))));
        }
    }

    // Mapping Starts to Grid Cells
    // A cell C_j is TRUE if there exists some block i such that s_i <= j < s_i + len_i
    for (j, cell_bool) in line.iter().enumerate() {
        let j_int = Int::from_i64(j as i64);
        
        let mut conditions = Vec::new();
        
        for i in 0..n_clues {
            let s = &starts[i];
            let len = clues[i] as i64;
            
            // Condition: s <= j AND j < s + len
            let after_start = s.le(&j_int);
            let before_end = j_int.lt(&(s + Int::from_i64(len)));
            
            // Use BitAnd for '&'
            use std::ops::BitAnd;
            conditions.push(after_start.bitand(&before_end));
        }
        
        // Constraint: cell_bool == OR(conditions)
        if conditions.is_empty() {
             let false_val = Bool::from_bool(false);
             // Use Ast::eq to compare logic, not equality of handles
             solver.assert(Ast::eq(*cell_bool, &false_val));
        } else {
             // Combine with OR
             use std::ops::BitOr;
             let mut combined_or = conditions[0].clone();
             for cond in conditions.iter().skip(1) {
                 combined_or = combined_or.bitor(cond);
             }
             solver.assert(Ast::eq(*cell_bool, &combined_or));
        }
    }
}
