# Deep Dive into the Solver

In Part 1, we saw the basic workflow: Define -> Assert -> Solve. In this part, we will explore the `Solver` in more depth, look at different data types, and solve a complex problem from Advent of Code.

## The Z3 Solver

The `Solver` is the workhorse of Z3. It maintains a stack of assertions and checks if there is *any* assignment of values to variables that satisfies all of them simultaneously.

### Key Concepts

1.  **Variable Types (Sorts):**
    *   `Int`: Mathematical integers (unbounded).
    *   `Real`: Real numbers (rational numbers).
    *   `Bool`: Boolean values (`true` / `false`).
    *   `BitVec`: Fixed-width bit vectors (useful for low-level systems programming models).

2.  **Satisfiability Results:**
    *   `Sat`: A solution (model) exists.
    *   `Unsat`: No solution exists. The assertions are contradictory.
    *   `Unknown`: The solver gave up (rare in simple problems, but happens with complex quantifiers or non-linear arithmetic).

3.  **The `Model`:**
    *   If the result is `Sat`, the solver provides a `Model`.
    *   A `Model` is essentially a map containing concrete values for each variable in your assertions.

### Finding Multiple Solutions

Z3 provides a helper method `solver.solutions()` to iterate over all satisfying models for a given set of variables. This simplifies the process of finding multiple solutions without manually managing blocking clauses.

**Iterator Approach:**
1.  Define your variables and constraints.
2.  Call `solver.solutions(vars, true)`.
3.  Iterate through the results.

*Note: Under the hood, this iterator still performs the "block and retry" strategy, but it handles the complexity for you.*

## Real-World Example: Advent of Code 2023 Day 24

We will use Z3 to solve [Advent of Code 2023 Day 24, Part 2](https://adventofcode.com/2023/day/24).

**The Problem:**
You are given the position `(px, py, pz)` and velocity `(vx, vy, vz)` of several hailstones. You need to throw a rock (with unknown starting position `rpx, rpy, rpz` and velocity `rvx, rvy, rvz`) such that it collides with **every** hailstone at some point in time `t`.

**The Math:**
For each hailstone $i$, there exists a time $t_i \ge 0$ such that:
$$
\begin{aligned}
rpx + rvx \cdot t_i &= px_i + vx_i \cdot t_i \\
rpy + rvy \cdot t_i &= py_i + vy_i \cdot t_i \\
rpz + rvz \cdot t_i &= pz_i + vz_i \cdot t_i
\end{aligned}
$$

This is a system of non-linear equations (since both the rock's velocity and the collision time are unknowns multiplied together). Z3's `Real` or `Int` solvers are excellent at handling this.

## Running the Code

```bash
car go run -p part_02_solver
```

## Homework: The Magic Square

**Goal:** Use the `distinct` constraint and matrix logic.

**Scenario:**
A **Magic Square** is a grid of numbers where the values in each row, each column, and both main diagonals sum to the same "magic constant".

**Task:**
Find a 3x3 Magic Square using digits 1 through 9.

**Constraints:**
1.  Grid is 3x3.
2.  Each cell contains a value between 1 and 9.
3.  All cells must be **distinct** (no repeats).
4.  Sum of each row = 15.
5.  Sum of each column = 15.
6.  Sum of diagonal (top-left to bottom-right) = 15.
7.  Sum of diagonal (top-right to bottom-left) = 15.

**Hints:**
*   You can define variables like `r0_c0`, `r0_c1`, etc., or use a `Vec` of `Vec`.
*   Z3 has a special helper: `z3::ast::Int::distinct(ctx, &[&x, &y, ...])`.
*   (Or in Rust z3 crate: `Ast::distinct(&[&x, &y, ...])`).
