# Introduction to Z3 & Setup

This project introduces the Z3 Theorem Prover and how to use it within Rust. We cover setting up the environment, understanding the core concepts of Z3, and writing your first solvers for Boolean logic and integer arithmetic.

## Setup & Installation

The `z3` crate in Rust is a wrapper around the Z3 C++ library. You need to have the Z3 library installed on your system for the crate to link against.

### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install libz3-dev
```

### macOS (Homebrew)

```bash
brew install z3
```

### Windows

You generally have two options:
1.  **Vcpkg:** Use `vcpkg` to install `z3`.
2.  **Pre-built Binaries:** Download pre-built binaries from the [Z3 releases page](https://github.com/Z3Prover/z3/releases), extract them, and ensure the `bin` folder is in your system's `PATH` and `lib` folder is accessible to the Rust linker.

### Rust Dependency

Add `z3` to your `Cargo.toml`. Since we are in a workspace, it's defined in the root `Cargo.toml`.
```toml
[dependencies]
z3 = "0.19.6"
```

### GitHub Token for `gh-release` (Optional)

If you are using the `gh-release` feature of the `z3` crate (e.g., via `cargo build --features gh-release`), you might encounter GitHub API rate limiting. To avoid this, you can set a GitHub Personal Access Token as an environment variable.

1.  **Generate a GitHub Token:**
    *   Go to your GitHub settings: `Settings > Developer settings > Personal access tokens > Tokens (classic)`
    *   Click "Generate new token" and select "Generate new token (classic)".
    *   Give your token a descriptive name (e.g., `z3-gh-release`).
    *   No specific scopes are needed for public repositories, but if you encounter issues, ensure `public_repo` or `repo` scopes are granted.
    *   Generate the token and copy it. **You will not be able to see it again.**

2.  **Set as Environment Variable:**
    Set the token as `GITHUB_TOKEN` in your shell environment. For example:

    ```bash
    export GITHUB_TOKEN="YOUR_GENERATED_TOKEN_HERE"
    ```
    For persistent setting, add this line to your shell's configuration file (e.g., `~/.bashrc`, `~/.zshrc`, or `~/.config/nushell/config.nu` for nushell users) and then `source` it or restart your terminal.

    **Note:** Replace `YOUR_GENERATED_TOKEN_HERE` with the actual token you copied from GitHub.

## What is Z3?

Z3 is a high-performance **SMT (Satisfiability Modulo Theories)** solver from Microsoft Research.

-   **Satisfiability (SAT):** Can we find values for variables that make a boolean formula true? (e.g., `x AND !y`).
-   **Modulo Theories (SMT):** Extends SAT to include background theories like arithmetic, bit-vectors, arrays, and uninterpreted functions. This allows us to solve equations like `x + y = 10`.

### Core Concepts

Solving a problem with Z3 generally follows these steps:

1.  **Define Variables**: Create symbolic variables (e.g., integers, booleans) that represent the unknowns in your problem.
2.  **Make Assertions**: Add logical constraints (assertions) to a `Solver` that describe the relationships and conditions these variables must satisfy.
3.  **Run the Solver**: Ask the `Solver` to find a solution (a set of values for the variables that satisfy all assertions). If a solution exists, it's called "satisfiable"; otherwise, it's "unsatisfiable."

### Z3 Workflow

Here's a general workflow for using Z3:

```
  +-------------------+
  | Create a Solver   |
  +-------------------+
          |
          V
  +-------------------+
  | Define Variables  |
  | (e.g., x, y, ...) |
  +-------------------+
          |
          V
  +-------------------+
  | Add Assertions    |
  | (Constraints)     |
  +-------------------+
          |
          V
  +-------------------+
  | Run Solver        |
  | (Check           |
  | Satisfiability)   |
  +-------------------+
          |
          V
  +-------------------+
  | Get Model         |
  | (If satisfiable)  |
  +-------------------+
```

## Code Walkthrough

We have implemented two examples in `src/main.rs`.

### Example 1: System of Equations
**Goal:** Solve the system:
$$
\begin{cases}
x + y = 10 \\
x - y = 2
\end{cases}
$$

**Logic:**
1.  Define Integer variables `x` and `y`.
2.  Create a solver instance.
3.  Assert `x + y == 10` to the solver.
4.  Assert `x - y == 2` to the solver.
5.  Check if the solver finds a solution and retrieve it.

### Example 2: Optimization
**Goal:** Minimize the value of $x + y$ subject to the following constraints:
1.  $x > 0$
2.  $y > 0$
3.  $2x + y \ge 10$

**Logic:**
1.  Define Integer variables `x` and `y`.
2.  Create an **Optimizer** instance (different from a standard Solver).
3.  Add the constraints (assertions) to the optimizer.
4.  Tell the optimizer to **minimize** the expression `x + y`.
5.  Check for satisfiability and retrieve the optimal model.

## Running the Code

To run this specific part of the tutorial series:

```bash
cargo run -p part_01_intro
```
## Further Reading

*   **Z3 Theorem Prover Official Website:** [https://github.com/Z3Prover/z3](https://github.com/Z3Prover/z3)
*   **Satisfiability Modulo Theories (SMT) on Wikipedia:** [https://en.wikipedia.org/wiki/Satisfiability_modulo_theories](https://en.wikipedia.org/wiki/Satisfiability_modulo_theories)
*   **Boolean Satisfiability Problem (SAT) on Wikipedia:** [https://en.wikipedia.org/wiki/Boolean_satisfiability_problem](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem)

## Homework: The Bakery Receipt

**Goal:** Solve a simple integer constraint problem.

**Scenario:**
You just left a bakery with a receipt for exactly **$50.00**. You bought three types of items:
*   **Croissants:** $3 each.
*   **Bagels:** $2 each.
*   **Muffins:** $4 each.

**Constraints:**
1.  You bought at least one of each item.
2.  You bought more Bagels than Muffins.
3.  The total number of items bought is 20.

**Task:**
Write a Z3 solver to find out exactly how many Croissants, Bagels, and Muffins you bought.

**Hints:**
*   Create three `Int` variables: `c`, `b`, `m`.
*   Assert `3*c + 2*b + 4*m == 50`.
*   Assert `c + b + m == 20`.
*   Don't forget the inequalities!

