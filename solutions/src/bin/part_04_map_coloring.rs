use z3::ast::Int;
use z3::{SatResult, Solver};

// 1. Graph Struct
// Encapsulates the topology of the graph (Nodes and Edges)
struct Graph {
    num_nodes: usize,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    // Creates the Petersen Graph specifically
    fn new_petersen() -> Self {
        Self {
            num_nodes: 10,
            edges: vec![
                // Outer Cycle
                (0, 1), (1, 2), (2, 3), (3, 4), (4, 0),
                // Spokes
                (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),
                // Inner Star (5-7-9-6-8-5)
                (5, 7), (7, 9), (9, 6), (6, 8), (8, 5),
            ],
        }
    }
}

// 2. Map Struct
// Encapsulates the Problem Form (Variables) and Logic
struct Map {
    graph: Graph,
    regions: Vec<Int>,
}

impl Map {
    fn new(graph: Graph) -> Self {
        let mut regions = Vec::new();
        for i in 0..graph.num_nodes {
            regions.push(Int::new_const(format!("Node_{}", i).as_str()));
        }
        Self { graph, regions }
    }

    // Applies the base constraints (Domain & Edges) to the solver
    fn init_constraints(&self, solver: &Solver) {
        // 1. Domain Constraints (Color is 1, 2, or 3)
        for r in &self.regions {
            // Using explicit i64 to help type inference
            solver.assert(&r.ge(1i64));
            solver.assert(&r.le(3i64));
        }

        // 2. Graph Edges
        for (u, v) in &self.graph.edges {
            solver.assert(&self.regions[*u].eq(&self.regions[*v]).not());
        }
    }
}

// 3. ColoringSolver Struct
// Encapsulates the Solver and the Search Strategy
struct ColoringSolver {
    solver: Solver,
}

impl ColoringSolver {
    fn new() -> Self {
        Self {
            solver: Solver::new(),
        }
    }

    // The DFS Logic
    fn solve_dfs(&self, map: &Map) {
        println!("Starting search on Graph ({} nodes)...", map.graph.num_nodes);

        let num_regions = map.graph.num_nodes;
        let mut current_idx = 0;
        // Track which color we are currently trying for each region level
        // 0 means not tried yet, 1..3 means trying that color
        let mut attempt_stack: Vec<i64> = vec![0; num_regions];

        loop {
            // Success Condition
            if current_idx >= num_regions {
                println!("Solution found!");
                self.print_solution(map);
                break;
            }

            let region = &map.regions[current_idx];
            let mut found_valid_color = false;

            // Try next color
            let start_color = attempt_stack[current_idx] + 1;

            for color in start_color..=3 {
                self.solver.push(); // Checkpoint

                // Assert: Region = Color
                self.solver.assert(&region.eq(color));

                if self.solver.check() == SatResult::Sat {
                    // Good move!
                    attempt_stack[current_idx] = color;
                    found_valid_color = true;
                    current_idx += 1;
                    break;
                } else {
                    // Bad move, undo
                    self.solver.pop(1);
                }
            }

            // Backtracking
            if !found_valid_color {
                if current_idx == 0 {
                    println!("No solution exists.");
                    break;
                }

                // Reset current level attempts
                attempt_stack[current_idx] = 0;

                // Go back one level
                current_idx -= 1;

                // Undo the successful move from the previous level
                self.solver.pop(1);
            }
        }
    }

    fn print_solution(&self, map: &Map) {
        if self.solver.check() == SatResult::Sat {
            let model = self.solver.get_model().unwrap();
            println!("Region Colors:");
            for (i, r) in map.regions.iter().enumerate() {
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
    }
}

fn main() {
    println!("--- Homework: Map Coloring (Petersen Graph) ---");

    // Initialize Components
    let graph = Graph::new_petersen();
    let map = Map::new(graph);
    let coloring_solver = ColoringSolver::new();

    // Setup Constraints
    map.init_constraints(&coloring_solver.solver);

    // Run Search
    coloring_solver.solve_dfs(&map);
}