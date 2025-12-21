# Part 5: Putting it All Together

Welcome to the grand finale! You've learned the basics, explored solvers, and mastered optimization. Now it's time to put those skills to work on some real-world (and not-so-real-world) puzzles.

In this section, we have three fun challenges for you to solve using Z3. Each one is a standalone tool that reads its configuration from a JSON file.

## How to Run Programs

Each program accepts a path to a JSON file as an argument. If you want to see what the input data looks like, check out the files in the `inputs/` directory.

```bash
# General usage:
cargo run -p part_05_application --bin <tool_name> <path_to_json>
```

You can also pipe JSON data directly into the tools via `stdin` if you're feeling fancy!
```bash
cat inputs/restaurant_sample.json | cargo run -p part_05_application --bin restaurant
```

## 🍔 Challenge 1: The Great Family Dinner Debate

Choosing a restaurant for the whole family can be a nightmare. Everyone has different tastes, some have dietary restrictions, and there's always a budget to consider. Can you try to find the best restaurant for the family?

In this challenge, we use Z3 to find the perfect spot that:

1.  **Respects dietary needs:** Ensuring everyone has something to eat (like vegan options).
2.  **Stays on budget:** No one wants to break the bank.
3.  **Maximizes happiness:** Finding the place with the highest combined ratings from the group.

**Template:** `src/bin/restaurant.rs`

**Try it out:**

```bash
# Find the best restaurant from our sample list:
cargo run -p part_05_application --bin restaurant inputs/restaurant_sample.json
```

## 🧩 Challenge 2: Logic Art (Nonograms)

Nonograms are those satisfying puzzles where you fill in cells on a grid to reveal a hidden picture. It's all about logic—if you follow the clues, the picture reveals itself.

We've built a solver that takes the row and column clues and lets Z3 figure out the image for us. It's a great example of how Z3 handles grid-based logic and constraints.

**Template:** `src/bin/nonogram.rs`

**Try it out:**

```bash
# Solve a 3x3 smiley face:
cargo run -p part_05_application --bin nonogram inputs/nonogram_smiley.json
```

## 🤖 Challenge 3: The Nanobot Rescue

Inspired by a classic Advent of Code puzzle, this challenge drops us into a 3D field filled with nanobots, each with its own transmission range.

Your mission: find the single point in 3D space that is within range of the **most** nanobots possible. We use "Manhattan Distance" to calculate ranges, turning our Z3 optimizer into a powerful geometric search tool.

**Template:** `src/bin/nanobots.rs`

**Try it out:**

```bash
# Find the most crowded spot in the nanobot field:
cargo run -p part_05_application --bin nanobots inputs/nanobots_sample.json
```

