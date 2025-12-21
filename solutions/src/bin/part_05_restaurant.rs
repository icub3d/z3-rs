use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use z3::ast::{Bool, Int};
use z3::{Optimize, SatResult};

#[derive(Deserialize, Debug)]
struct InputData {
    budget: i64,
    restaurants: Vec<Restaurant>,
    people: Vec<Person>,
}

#[derive(Deserialize, Debug)]
struct Restaurant {
    name: String,
    cost: i64,
    vegan: bool,
}

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    is_vegan: bool,
    ratings: Vec<i64>,
}

fn get_input() -> InputData {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn Read> = if args.len() > 1 {
        Box::new(File::open(&args[1]).expect("Failed to open file"))
    } else {
        Box::new(io::stdin())
    };

    serde_json::from_reader(reader).expect("Failed to parse JSON input")
}

fn main() {
    let data = get_input();
    let opt = Optimize::new();

    // One-hot encoding: Boolean variable for each restaurant
    let is_chosen: Vec<Bool> = (0..data.restaurants.len())
        .map(|i| Bool::new_const(format!("restaurant_{}", i)))
        .collect();

    // Exactly one restaurant must be chosen
    let constraints: Vec<(&Bool, i32)> = is_chosen.iter().map(|b| (b, 1)).collect();
    // NOTE: This allows us to potentially pick the "best N". Here we choose 1.
    opt.assert(&Bool::pb_eq(&constraints, 1));

    // Build happiness terms and add budget constraints for each restaurant
    let happiness_terms: Vec<Int> = data
        .restaurants
        .iter()
        .enumerate()
        .map(|(i, restaurant)| {
            // Budget constraint: if this restaurant is chosen, cost must be within budget
            let total_cost = restaurant.cost * data.people.len() as i64;
            opt.assert(&is_chosen[i].implies(Int::from_i64(total_cost).le(data.budget)));

            let restaurant_happiness: i64 = data
                .people
                .iter()
                .map(|person| {
                    if person.is_vegan && !restaurant.vegan {
                        0
                    } else {
                        person.ratings[i]
                    }
                })
                .sum();

            // If chosen, contribute happiness; otherwise contribute 0
            is_chosen[i].ite(&Int::from_i64(restaurant_happiness), &Int::from_i64(0))
        })
        .collect();

    // Sum up all happiness terms (only one will be non-zero)
    let terms_refs: Vec<&Int> = happiness_terms.iter().collect();
    let total_happiness = Int::add(&terms_refs);

    // Maximize happiness
    opt.maximize(&total_happiness);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let chosen_idx = is_chosen
            .iter()
            .position(|var| model.eval(var, true).unwrap().as_bool().unwrap())
            .unwrap();

        println!(
            "Selected Restaurant: {} (Index {})",
            data.restaurants[chosen_idx].name, chosen_idx
        );
        println!(
            "Total Happiness: {}",
            model.eval(&total_happiness, true).unwrap()
        );
        println!("Cost per head: ${}", data.restaurants[chosen_idx].cost);

        println!("\nIndividual Happiness:");
        for person in &data.people {
            let happiness = if person.is_vegan && !data.restaurants[chosen_idx].vegan {
                0
            } else {
                person.ratings[chosen_idx]
            };
            println!("  {}: {}", person.name, happiness);
        }
    } else {
        println!("No suitable restaurant found within budget/constraints.");
    }
}
