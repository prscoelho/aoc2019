use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Ingredient {
    name: String,
    quantity: u64,
}

impl Ingredient {
    fn parse(quantity: &str, name: &str) -> Ingredient {
        Ingredient {
            name: name.into(),
            quantity: quantity.parse().unwrap(),
        }
    }

    fn new(quantity: u64, name: String) -> Ingredient {
        Ingredient { name, quantity }
    }
    fn mul(&self, num: u64) -> Ingredient {
        Ingredient::new(self.quantity * num, self.name.clone())
    }
}

#[derive(Debug)]
struct Reaction {
    produces: Ingredient,
    requires: Vec<Ingredient>,
}

impl Reaction {
    fn new(produces: Ingredient, requires: Vec<Ingredient>) -> Reaction {
        Reaction { produces, requires }
    }
}

fn read_reactions(input: &str) -> HashMap<String, Reaction> {
    let mut reactions = HashMap::new();
    // split left => right
    let line_regex = Regex::new(r"(?m)^(.+?)=> (.+?)$").unwrap();
    // parse <quantity> <ingredient>
    let ingredient_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    for line in line_regex.captures_iter(input) {
        let mut requires = Vec::new();
        let left = &line[1];
        let right = &line[2];

        for ingredient_capture in ingredient_regex.captures_iter(left) {
            requires.push(Ingredient::parse(
                &ingredient_capture[1],
                &ingredient_capture[2],
            ));
        }
        let produces_capture = ingredient_regex.captures(right).unwrap();
        let produces = Ingredient::parse(&produces_capture[1], &produces_capture[2]);

        reactions.insert(produces.name.clone(), Reaction::new(produces, requires));
    }

    reactions
}

// When attempting to use leftovers, there are three possible scenarios:
// 1. There was enough quantity of ingredient in the bag, so there's no need to create ingredient. Ingredient quantity is subtracted from leftovers.
// 2. There wasn't any ammount of ingredient in the bag, and there's need to create ingredient.
// 3. There was some quantity of ingredient, but it was insufficient. There's still need to create ingredient, but less quantity of it.
// try_leftovers returns the ammount of quantity remaining to produce ingredient.
fn try_leftovers(to_produce: &Ingredient, leftovers: &mut HashMap<String, u64>) -> u64 {
    if let Some(quantity) = leftovers.get_mut(&to_produce.name) {
        // scenario 1
        if to_produce.quantity <= *quantity {
            *quantity -= to_produce.quantity;
            return 0;
        } else {
            // scenario 3
            let remaining = to_produce.quantity - *quantity;
            leftovers.remove(&to_produce.name);
            return remaining;
        }
    } else {
        // scenario 2
        to_produce.quantity
    }
}

fn fuel_cost(reactions: &HashMap<String, Reaction>, fuel_ammount: u64) -> u64 {
    let fuel = Ingredient::new(fuel_ammount, String::from("FUEL"));
    let mut ore = 0;
    let mut requirement_list = vec![fuel];
    let mut leftovers: HashMap<String, u64> = HashMap::new();
    while let Some(required) = requirement_list.pop() {
        if required.name == "ORE" {
            ore += required.quantity;
        } else {
            // attempt to use from leftovers
            let remaining = try_leftovers(&required, &mut leftovers);
            if remaining == 0 {
                continue;
            }
            let reaction = reactions
                .get(&required.name)
                .expect("Reaction to produce ingredient does not exist.");
            let reaction_multiplier =
                (remaining as f64 / reaction.produces.quantity as f64).ceil() as u64;
            // add leftovers -- before actually going down the list of requirements
            // hopefully nothing breaks?
            if let Some(result) =
                (reaction.produces.quantity * reaction_multiplier).checked_sub(remaining)
            {
                if result > 0 {
                    *leftovers.entry(required.name).or_insert(0) += result;
                }
            }

            for requirement in reaction.requires.iter() {
                requirement_list.push(requirement.mul(reaction_multiplier));
            }
        }
    }
    ore
}

pub fn solve_first(input: &str) -> u64 {
    let reactions = read_reactions(input);

    fuel_cost(&reactions, 1)
}

// perform a binary search that finds the highest ammount of fuel that can be generated from target ore
fn max_fuel(reactions: &HashMap<String, Reaction>, target: u64) -> u64 {
    let cost_one = fuel_cost(&reactions, 1);
    let mut fuel_left = target / cost_one;
    let mut fuel_right = fuel_left * 2;
    while fuel_right - fuel_left > 1 {
        let fuel = (fuel_left + fuel_right) / 2;
        let cost = fuel_cost(&reactions, fuel);

        if cost < target {
            fuel_left = fuel;
        } else {
            fuel_right = fuel;
        }
    }
    // fuel_right requires more ore than target
    // fuel left is the highest ammount of fuel without going over target
    fuel_left
}

pub fn solve_second(input: &str) -> u64 {
    let reactions = read_reactions(input);
    max_fuel(&reactions, 1_000_000_000_000)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example3_2() {
        let input = include_str!("example3");
        assert_eq!(solve_second(input), 82892753);
    }

    #[test]
    fn test_example4_2() {
        let input = include_str!("example4");
        assert_eq!(solve_second(input), 5586022);
    }

    #[test]
    fn test_example5_2() {
        let input = include_str!("example5");
        assert_eq!(solve_second(input), 460664);
    }

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 278404);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 4436981);
    }
}
