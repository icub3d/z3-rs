use z3::ast::Int;
use z3::{SatResult, Solver};

fn main() {
    println!("--- Homework: Map Coloring (Petersen Graph) ---");

    let solver = Solver::new();

    // 10 Regions (0..9)
    let num_regions = 10;
    let mut regions = Vec::new();
    for i in 0..num_regions {
        regions.push(Int::new_const(format!("Node_{}", i).as_str()));
    }

    // 1. Domain Constraints (Color is 1, 2, or 3)
    for r in &regions {
        solver.assert(r.ge(1));
        solver.assert(r.le(3));
    }

    // 2. Graph Edges (Petersen Graph)
    let edges = vec![
        // Outer Cycle
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 0),
        // Spokes
        (0, 5),
        (1, 6),
        (2, 7),
        (3, 8),
        (4, 9),
        // Inner Star (5-7-9-6-8-5)
        (5, 7),
        (7, 9),
        (9, 6),
        (6, 8),
        (8, 5),
    ];

    for (u, v) in edges {
        solver.assert(regions[u].eq(&regions[v]).not());
    }

    // 3. Programmatic Search Strategy (Depth-First Search)
    println!("Starting search on Petersen Graph (10 nodes)...");

    let mut current_region_idx = 0;
    // Track which color we are currently trying for each region level
    // 0 means not tried yet, 1..3 means trying that color
    let mut attempt_stack = vec![0; num_regions];

    loop {
        if current_region_idx >= num_regions {
            println!("Solution found!");
            if solver.check() == SatResult::Sat {
                let model = solver.get_model().unwrap();
                println!("Region Colors:");
                for (i, r) in regions.iter().enumerate() {
                    let c_val = model.eval(r, true).unwrap().as_i64().unwrap();
                    let c_name = match c_val {
                        1 => "Red",
                        2 => "Green",
                        3 => "Blue",
                        _ => "Unknown",
                    };
                    println!("  Node {}: {} ({})", i, c_name, c_val);
                }
            }
            break;
        }

        let region = &regions[current_region_idx];
        let mut found_valid_color = false;

        // Try next color
        let start_color = attempt_stack[current_region_idx] + 1;

        for color in start_color..=3 {
            solver.push(); // Checkpoint

            // Assert: Region = Color
            solver.assert(region.eq(Int::from_i64(color)));

            if solver.check() == SatResult::Sat {
                // Good move!
                attempt_stack[current_region_idx] = color;
                found_valid_color = true;
                current_region_idx += 1;
                break;
            } else {
                // Bad move, undo
                solver.pop(1);
            }
        }

        if !found_valid_color {
            // Backtrack!
            if current_region_idx == 0 {
                println!("No solution exists.");
                break;
            }

            // Reset current level attempts
            attempt_stack[current_region_idx] = 0;

            // Go back one level
            current_region_idx -= 1;

            // Undo the successful move from the previous level
            solver.pop(1);
        }
    }
}
