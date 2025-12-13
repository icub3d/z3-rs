# Part 5: Application Challenges

Welcome to the final part of the series! This section provides three **challenge problems** implemented as command-line tools.

The tools parse input from a **JSON file** (provided as an argument) or from **stdin**.

## Challenge 1: Family Restaurant Picker

This problem is a classic **Constraint Satisfaction Problem (CSP)** with an optimization component. We need to find a single variable assignment (the restaurant) that satisfies strict logical constraints (budget) and weighted preferences (happiness).

*   **See also:** [Constraint Satisfaction Problem (Wikipedia)](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem)

**File:** `src/bin/restaurant.rs`

**Input Format (JSON):**
```json
{
  "budget": 150,
  "restaurants": [
    { "name": "SteakHouse", "cost": 50, "vegan": false },
    { "name": "VeganSpot", "cost": 20, "vegan": true }
  ],
  "people": [
    { "name": "Dad", "is_vegan": false, "ratings": [10, 2] },
    { "name": "Kid", "is_vegan": true, "ratings": [0, 10] }
  ]
}
```

**Running:**
```bash
# Using the sample input:
cargo run -p part_05_application --bin restaurant inputs/restaurant_sample.json

# Using the tight budget input:
cargo run -p part_05_application --bin restaurant inputs/restaurant_tight.json

# Using a solvable input:
cargo run -p part_05_application --bin restaurant inputs/restaurant_solvable.json
```

---

## Challenge 2: Nonogram Solver

Nonograms (also known as Hanjie, Picross, or Griddlers) are picture logic puzzles in which cells in a grid must be colored or left blank according to numbers at the side of the grid to reveal a hidden picture.

*   **See also:** [Nonogram (Wikipedia)](https://en.wikipedia.org/wiki/Nonogram)

**File:** `src/bin/nonogram.rs`

**Input Format (JSON):**
```json
{
  "rows": 3,
  "cols": 3,
  "row_clues": [[1], [1], [1]],
  "col_clues": [[1], [1], [1]]
}
```

**Running:**
```bash
# Solve the smiley face puzzle:
cargo run -p part_05_application --bin nonogram inputs/nonogram_smiley.json

# Solve the cross puzzle:
cargo run -p part_05_application --bin nonogram inputs/nonogram_cross.json
```

---

## Challenge 3: Nanobot Range Optimizer

This challenge is inspired by Advent of Code 2018 Day 23. It involves finding a coordinate in 3D space that is within range of the maximum number of "nanobots". The distance metric used is **Manhattan Distance** ($|x_1 - x_2| + |y_1 - y_2| + |z_1 - z_2|$), which creates diamond-shaped "spheres" in geometry.

*   **See also:** [Taxicab Geometry (Manhattan Distance) (Wikipedia)](https://en.wikipedia.org/wiki/Taxicab_geometry)

**File:** `src/bin/nanobots.rs`

**Input Format (JSON):**
```json
{
  "bots": [
    { "x": 10, "y": 12, "z": 12, "r": 2 },
    { "x": 12, "y": 14, "z": 12, "r": 2 }
  ]
}
```

**Running:**
```bash
# Using the sample input:
cargo run -p part_05_application --bin nanobots inputs/nanobots_sample.json

# Using the simple input:
cargo run -p part_05_application --bin nanobots inputs/nanobots_simple.json
```