use z3::ast::{Bool, Int};
use z3::{Optimize, SatResult};

/// Example 1: The Knapsack Problem (Maximization)
/// Goal: Maximize value of items taken without exceeding weight limit.
///   - Weight Limit: 15
///   - Item A: Val 4, Wgt 12
///   - Item B: Val 2, Wgt 2
///   - Item C: Val 2, Wgt 1
///   - Item D: Val 1, Wgt 1
///   - Item E: Val 10, Wgt 4
fn demonstrate_maximization() {
    println!("\n--- Knapsack Problem (Maximization) ---");

    let opt = Optimize::new();

    struct Item {
        name: &'static str,
        val: i64,
        wgt: i64,
    }
    let items = [
        Item {
            name: "A",
            val: 4,
            wgt: 12,
        },
        Item {
            name: "B",
            val: 2,
            wgt: 2,
        },
        Item {
            name: "C",
            val: 2,
            wgt: 1,
        },
        Item {
            name: "D",
            val: 1,
            wgt: 1,
        },
        Item {
            name: "E",
            val: 10,
            wgt: 4,
        },
    ];

    let limit = 15;

    // Define Decision Variables (Bool: Taken/Not Taken)
    let taken: Vec<Bool> = items
        .iter()
        .map(|item| Bool::new_const(item.name))
        .collect();

    // Build Expressions for Total Weight and Total Value
    let mut total_weight = Int::from_i64(0);
    let mut total_value = Int::from_i64(0);

    for (i, item) in items.iter().enumerate() {
        let is_taken = &taken[i];

        // Z3 allows if condition { value } else { value} with ite.
        let wgt = is_taken.ite(&Int::from_i64(item.wgt), &Int::from_i64(0));
        let val = is_taken.ite(&Int::from_i64(item.val), &Int::from_i64(0));

        total_weight = &total_weight + &wgt;
        total_value = &total_value + &val;
    }

    // Constraint: Total Weight <= Limit
    opt.assert(&total_weight.le(Int::from_i64(limit)));

    // Objective: Maximize Total Value
    opt.maximize(&total_value);

    // Solve
    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        println!("Optimal Knapsack Selection:");
        for (i, item) in items.iter().enumerate() {
            if model.eval(&taken[i], true).unwrap().as_bool().unwrap() {
                println!(
                    "  - Taken: {} (Val: {}, Wgt: {})",
                    item.name, item.val, item.wgt
                );
            }
        }
        println!("  Total Value: {}", model.eval(&total_value, true).unwrap());
        println!(
            "  Total Weight: {}",
            model.eval(&total_weight, true).unwrap()
        );
    } else {
        println!("UNSAT");
    }
}

/// Example 2: Advent of Code 2025 Day 10 Part 2 (Factory)
/// Goal: Minimize button presses to reach target joltage levels.
fn solve_aoc_day10() {
    println!("\n--- AoC Day 10 Part 2 ---");

    struct Machine {
        targets: Vec<i64>,
        buttons: Vec<Vec<usize>>,
    }

    // Example Input Data
    let machines = [
        // Machine 1: {3,5,4,7}
        // (3), (1,3), (2), (2,3), (0,2), (0,1)
        Machine {
            targets: vec![3, 5, 4, 7],
            buttons: vec![
                vec![3],    // Button 0
                vec![1, 3], // Button 1
                vec![2],    // Button 2
                vec![2, 3], // Button 3
                vec![0, 2], // Button 4
                vec![0, 1], // Button 5
            ],
        },
        // Machine 2: {7,5,12,7,2}
        // (0,2,3,4), (2,3), (0,4), (0,1,2), (1,2,3,4)
        Machine {
            targets: vec![7, 5, 12, 7, 2],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
        },
        // Machine 3: {10,11,11,5,10,5}
        // (0,1,2,3,4), (0,3,4), (0,1,2,4,5), (1,2)
        Machine {
            targets: vec![10, 11, 11, 5, 10, 5],
            buttons: vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 3, 4],
                vec![0, 1, 2, 4, 5],
                vec![1, 2],
            ],
        },
    ];

    let mut total_presses_all_machines = 0;

    for (m_idx, machine) in machines.iter().enumerate() {
        // Create a fresh optimizer for each machine
        let opt = Optimize::new();

        // Define Variables: presses for each button
        let presses: Vec<Int> = (0..machine.buttons.len())
            .map(|i| Int::new_const(format!("p_{}_{}", m_idx, i)))
            .collect();

        // Basic Constraint: Presses must be non-negative
        for p in &presses {
            opt.assert(&p.ge(Int::from_i64(0)));
        }

        // Target Constraints
        // For each counter (target), Sum(presses of buttons affecting it) == target
        for (c_idx, &target_val) in machine.targets.iter().enumerate() {
            let mut current_sum = Int::from_i64(0);

            for (b_idx, button_affects) in machine.buttons.iter().enumerate() {
                if button_affects.contains(&c_idx) {
                    current_sum = &current_sum + &presses[b_idx];
                }
            }

            opt.assert(&current_sum.eq(Int::from_i64(target_val)));
        }

        // Objective: Minimize total presses
        let mut sum_presses = Int::from_i64(0);
        for p in &presses {
            sum_presses = &sum_presses + p;
        }
        opt.minimize(&sum_presses);

        // Solve
        if opt.check(&[]) == SatResult::Sat {
            let model = opt.get_model().unwrap();
            let min_presses = model.eval(&sum_presses, true).unwrap().as_i64().unwrap();
            println!("Machine {}: Min presses = {}", m_idx + 1, min_presses);
            total_presses_all_machines += min_presses;
        } else {
            println!("Machine {}: UNSAT", m_idx + 1);
        }
    }

    println!(
        "Total minimum presses required: {}",
        total_presses_all_machines
    );
}

/// Represents a person's preference for a meeting time
struct Preference {
    time: i64,
    weight: u32,
}

/// Solves a meeting scheduling problem with soft constraints
fn solve_meeting_schedule(scenario: &str, preferences: &[Preference], explanation: &str) {
    println!("\n{}", scenario);

    let opt = Optimize::new();
    let time = Int::new_const("meeting_time");

    // Hard constraints: meeting must be between 9 and 11 AM
    opt.assert(&time.ge(Int::from_i64(9)));
    opt.assert(&time.le(Int::from_i64(11)));

    // Soft constraints: add each person's preference
    // Use the same group so penalties are summed
    for pref in preferences {
        opt.assert_soft(
            &time.eq(Int::from_i64(pref.time)),
            pref.weight,
            Some("preferences".into()),
        );
    }

    // Z3 automatically minimizes soft constraint violations
    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let selected_time = model.eval(&time, true).unwrap().as_i64().unwrap();
        println!("  Selected Time: {} AM", selected_time);
        println!("  {}", explanation);
    }
}

/// Example 3: Soft Constraints
/// Goal: Schedule a meeting where participants have conflicting preferences.
/// Soft constraints allow expressing preferences that should be satisfied if possible.
/// Z3 minimizes the sum of weights of violated soft constraints.
fn demonstrate_soft_constraints() {
    println!("\n--- Soft Constraints (Meeting Scheduling) ---");

    // Scenario 1: Two conflicting preferences with equal weight
    solve_meeting_schedule(
        "Scenario 1: Alice (9 AM, weight 10) vs Bob (10 AM, weight 10)",
        &[
            Preference {
                time: 9,
                weight: 10,
            },
            Preference {
                time: 10,
                weight: 10,
            },
        ],
        "(Equal weights: Z3 picks arbitrarily between 9 or 10)",
    );

    // Scenario 2: Unequal weights - boss overrules
    solve_meeting_schedule(
        "Scenario 2: Alice (9 AM, weight 10) vs Boss (10 AM, weight 50)",
        &[
            Preference {
                time: 9,
                weight: 10,
            },
            Preference {
                time: 10,
                weight: 50,
            },
        ],
        "(Boss's weight 50 > Alice's 10, so Boss's preference wins)",
    );

    // Scenario 3: Three people with one weighted heavily
    println!("\nScenario 3: Alice (9 AM), Bob (10 AM), Charlie (11 AM) all weight 10,");
    println!("            Boss (10 AM, weight 50) joins Bob");
    solve_meeting_schedule(
        "",
        &[
            Preference {
                time: 9,
                weight: 10,
            },
            Preference {
                time: 10,
                weight: 10,
            },
            Preference {
                time: 11,
                weight: 10,
            },
            Preference {
                time: 10,
                weight: 50,
            },
        ],
        "(10 AM: violate Alice=10 + Charlie=10 = 20 penalty - BEST!)",
    );
}

fn main() {
    demonstrate_maximization();
    solve_aoc_day10();
    demonstrate_soft_constraints();
}
