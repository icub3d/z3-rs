# Part 5: Application Challenges

Welcome to the final part of the series! This section provides three **challenge problems** implemented as command-line tools.

The tools parse input from a **JSON file** (provided as an argument) or from **stdin**.

## Challenge 1: Family Restaurant Picker
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