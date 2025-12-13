use std::env;
use std::fs::File;
use std::io::{self, Read};
use z3::{Optimize, SatResult};
use z3::ast::Int;
use serde::Deserialize;

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
    #[serde(rename = "name")]
    _name: String,
    is_vegan: bool,
    ratings: Vec<i64>, // rating per restaurant index
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

    // 1. Variable: Chosen Restaurant Index
    let chosen_idx = Int::new_const("chosen_idx");

    // 2. Constraints
    // Range: 0 <= idx < num_restaurants
    opt.assert(&chosen_idx.ge(Int::from_i64(0)));
    opt.assert(&chosen_idx.lt(Int::from_i64(data.restaurants.len() as i64)));

    // Budget: total_cost <= budget
    // total_cost = num_people * cost_of_chosen
    // We need to map `chosen_idx` to `cost`.
    // cost = If(idx==0, cost0, If(idx==1, cost1, ...))
    
    // Helper to select value based on index
    // Note: A more efficient way in big problems is `Array` sort, but nested ITE is fine for small N.
    fn get_value_by_index(idx_var: &Int, values: &[i64], default: i64) -> Int {
        let mut expr = Int::from_i64(default);
        for (i, val) in values.iter().enumerate().rev() {
            let i_const = Int::from_i64(i as i64);
            let v_const = Int::from_i64(*val);
            expr = idx_var.eq(&i_const).ite(&v_const, &expr);
        }
        expr
    }
    
    // Map idx -> cost
    let costs: Vec<i64> = data.restaurants.iter().map(|r| r.cost).collect();
    let chosen_cost = get_value_by_index(&chosen_idx, &costs, 999999);
    
    let total_cost = &chosen_cost * (data.people.len() as i64);
    opt.assert(&total_cost.le(Int::from_i64(data.budget)));

    // 3. Objective: Maximize Happiness
    // Total Happiness = Sum of ratings for chosen restaurant
    // NOTE: Removed strict vegan constraint. Instead, if a person is vegan, 
    // their happiness is 0 if the chosen restaurant is not vegan.
    let mut total_happiness = Int::from_i64(0);
    
    for p in &data.people {
        // Adjust ratings if person is vegan: 0 happiness for non-vegan places
        let effective_ratings: Vec<i64> = if p.is_vegan {
             p.ratings.iter().enumerate().map(|(i, &r)| {
                 if data.restaurants[i].vegan { r } else { 0 }
             }).collect()
        } else {
             p.ratings.clone()
        };

        // Person's happiness = get_value_by_index(chosen_idx, effective_ratings)
        let p_happiness = get_value_by_index(&chosen_idx, &effective_ratings, 0);
        total_happiness = &total_happiness + &p_happiness;
    }

    opt.maximize(&total_happiness);

    // 4. Solve
    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let idx = model.eval(&chosen_idx, true).unwrap().as_i64().unwrap() as usize;
        let r_name = &data.restaurants[idx].name;
        let happy_val = model.eval(&total_happiness, true).unwrap();
        
        println!("Selected Restaurant: {} (Index {})", r_name, idx);
        println!("Total Happiness: {}", happy_val);
        println!("Cost per head: ${}", data.restaurants[idx].cost);

        println!("\nIndividual Happiness:");
        for p in &data.people {
            let effective_ratings: Vec<i64> = if p.is_vegan {
                 p.ratings.iter().enumerate().map(|(i, &r)| {
                     if data.restaurants[i].vegan { r } else { 0 }
                 }).collect()
            } else {
                 p.ratings.clone()
            };
            let person_happiness = effective_ratings[idx]; // Directly get the rating for the chosen index
            println!("  {}: {}", p._name, person_happiness);
        }
    } else {
        println!("No suitable restaurant found within budget/constraints.");
    }
}
