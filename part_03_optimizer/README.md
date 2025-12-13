# The Optimizer

In this part, we explore the `Optimize` struct, which extends the capabilities of the standard `Solver`. While a `Solver` asks "Is there *any* solution?", an `Optimizer` asks "What is the *best* solution?".

## Key Concepts

1.  **Objectives (`minimize`, `maximize`):
    *   Instead of just satisfying constraints, you can tell Z3 to find a model that minimizes or maximizes the value of an arithmetic expression.
    *   Example: `opt.minimize(&(&x + &y))` tries to make the sum $x+y$ as small as possible.

2.  **Hard Constraints (`assert`):
    *   These work exactly like in the standard `Solver`. They *must* be true for any solution.
    *   If a hard constraint cannot be satisfied, the result is `Unsat`.

3.  **Soft Constraints (`assert_soft`):
    *   These are optional. The optimizer tries to satisfy as many as possible.
    *   You can assign **weights** to soft constraints. Z3 will minimize the sum of weights of *unsatisfied* soft constraints.
    *   This is useful for problems where perfect solutions might not exist, or where you want to express preferences.

## Code Walkthrough

We have implemented three examples in `src/main.rs`.

### Example 1: Basic Minimization

**Goal:** Minimize $x + y$ subject to $x > 0$, $y > 0$, and $2x + y \ge 10$.

**Logic:**
1.  Define variables `x`, `y`.
2.  Add hard constraints.
3.  Call `optimizer.minimize(&(x + y))`.

### Example 2: The Knapsack Problem

**Goal:** You have a backpack with weight limit 15. You want to choose items to maximize value without exceeding the weight.

*   Item A: Value 4, Weight 12
*   Item B: Value 2, Weight 2
*   Item C: Value 2, Weight 1
*   Item D: Value 1, Weight 1
*   Item E: Value 10, Weight 4

**Logic:**
1.  Define boolean variables for each item (Taken / Not Taken).
2.  Constraint: Total Weight <= 15.
3.  Objective: Maximize Total Value.

### Example 3: Advent of Code Day 10 Part 2 (Factory)

**Goal:** Minimize the total number of button presses required to set machine counters to specific target values.

**Problem:**
*   Each machine has several counters (initially 0).
*   Buttons increment specific counters by 1.
*   We need to reach specific target values for each counter.
*   We want to minimize $\sum (\text{presses for each button})$.

**Logic:**
1.  For each button on a machine, define an integer variable `p_i` (number of presses).
2.  Constraint: `p_i >= 0`.
3.  For each counter `c_j`, the sum of presses for all buttons affecting `c_j` must equal the target value for `c_j`.
4.  Objective: Minimize $\sum p_i$.

## Running the Code

```bash
car go run -p part_03_optimizer
```

## Homework: Production Planning

**Goal:** Maximize profit with resource constraints.

**Scenario:**
Your factory produces two products: **Chairs** and **Tables**.

*   **Chairs:**
    *   Sell for **$20**.
    *   Require **1 hour** of carpentry.
    *   Require **3 hours** of painting.
*   **Tables:**
    *   Sell for **$50**.
    *   Require **4 hours** of carpentry.
    *   Require **1 hour** of painting.

**Resources Available:**
*   Carpentry: **40 hours** max per week.
*   Painting: **40 hours** max per week.

**Task:**
Determine the optimal number of Chairs and Tables to produce to **maximize revenue**.

**Hints:**
*   Use `Optimize::new()`.
*   Variables `c` (chairs) and `t` (tables) must be >= 0.
*   Add constraints for total carpentry time and total painting time.
*   Maximize `20*c + 50*t`.

