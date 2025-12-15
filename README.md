# Z3 with Rust Mini-Series

This repository contains the code for a tutorial series on using the Z3 Theorem Prover with Rust.

## Series Outline

### Part 1: Introduction to Z3 & Setup
- **Topic:** What is Z3? Getting Started
- **Goal:** Learn a bit about how Z3 works with some basic examples.
- **Project:** `part_01_intro`
- **Key Concepts:** `Solver`, `Optimizer`, `Int`, `assert`, `check`, `get_model`.

### Part 2: The Solver
- **Topic:** Deep dive into the generic `Solver`.
- **Key Concepts:** Multiple Solutions, Complex Problems.
- **Project:** `part_02_solver`

### Part 3: The Optimizer
- **Topic:** Using the `Optimize`.
- **Key Concepts:** Hard vs. Soft constraints, Minimization, Maximization.
- **Project:** `part_03_optimizer`

### Part 4: Incremental Solving
- **Topic:** Advanced control over the solver state.
- **Key Concepts:** `push()`, `pop()`, scopes, and backtracking.
- **Project:** `part_04_push_pop`

### Part 5: Showcase
- **Topic:** Leverage Skills to Solve Problems.
- **Goal:** Solve three problems to test your skills.
- **Project:** `part_05_application`

## Running the Examples
You can run any part of the series using cargo:

```bash
cargo run -p part_01_intro
cargo run -p part_02_solver
# ... and so on
```
