use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

static OUTER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) => (.*)").unwrap());
static COMPONENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]*) ([A-Z]*)").unwrap());

static ONE_TRILLION: u64 = 1_000_000_000_000;

#[derive(PartialEq, Debug, Clone)]
struct Recipe {
    inputs: Vec<RecipeComponent>,
    output: RecipeComponent,
}

impl Recipe {
    pub fn new(recipe: &str) -> Recipe {
        let captures = OUTER_RE.captures(recipe).unwrap();
        let inputs = captures[1].split(", ").map(RecipeComponent::new).collect();

        Recipe {
            inputs,
            output: RecipeComponent::new(&captures[2]),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct RecipeComponent {
    chemical: String,
    quantity: u64,
}

impl RecipeComponent {
    pub fn new(component: &str) -> RecipeComponent {
        let captures = COMPONENT_RE.captures(component).unwrap();

        RecipeComponent {
            chemical: captures[2].to_string(),
            quantity: captures[1].parse().unwrap(),
        }
    }
}

pub fn fourteen_a() -> u64 {
    let recipes = load_recipes("src/inputs/14.txt");
    5
}

fn ore_cost_for_fuel(recipes: &HashMap<String, Recipe>, fuel_quantity: u64) -> u64 {
    5
}

fn num_fuel_producible_with_one_trillion_ore(recipes: &HashMap<String, Recipe>) -> u64 {
    let mut lower_bound = ONE_TRILLION / ore_cost_for_fuel(&recipes, 1);
    let mut upper_bound = 10 * lower_bound;

    while ore_cost_for_fuel(&recipes, upper_bound) < ONE_TRILLION {
        dbg!(upper_bound, ore_cost_for_fuel(&recipes, upper_bound));
        lower_bound = upper_bound;
        upper_bound *= 10;
    }

    loop {
        let midpoint = (lower_bound + upper_bound) / 2;
        println!("midpoint is {}", midpoint);
        let cost = ore_cost_for_fuel(&recipes, midpoint);
        dbg!(cost);

        if cost <= ONE_TRILLION && ore_cost_for_fuel(&recipes, midpoint + 1) > ONE_TRILLION {
            println!(
                "ding ding ding, cost of one more is {}",
                ore_cost_for_fuel(&recipes, midpoint + 1)
            );
            return midpoint;
        }

        if cost < ONE_TRILLION {
            println!("setting lower bound to {}", midpoint);
            lower_bound = midpoint;
        } else {
            println!("setting upper bound to {}", midpoint);
            upper_bound = midpoint;
        }
    }
}

/// "Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?"
pub fn fourteen_b() -> u64 {
    let recipes = load_recipes("src/inputs/14.txt");
    num_fuel_producible_with_one_trillion_ore(&recipes)
}

fn load_recipes(filename: &str) -> HashMap<String, Recipe> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(Recipe::new)
        .map(|recipe| (recipe.output.chemical.clone(), recipe))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_recipe() {
        assert_eq!(
            Recipe::new("7 LCSV, 1 LKPNB, 36 CMNH, 1 JZXPH, 20 DGJPN, 3 WDWB, 69 DXJKC, 3 WHJKH, 18 XSGP, 22 CGZL, 2 BNVB, 57 PNSD => 1 FUEL"),
            Recipe {inputs: vec![RecipeComponent { chemical: "LCSV".to_string(), quantity: 7 }, RecipeComponent { chemical: "LKPNB".to_string(), quantity: 1 }, RecipeComponent { chemical: "CMNH".to_string(), quantity: 36 }, RecipeComponent { chemical: "JZXPH".to_string(), quantity: 1 }, RecipeComponent { chemical: "DGJPN".to_string(), quantity: 20 }, RecipeComponent { chemical: "WDWB".to_string(), quantity: 3 }, RecipeComponent { chemical: "DXJKC".to_string(), quantity: 69 }, RecipeComponent { chemical: "WHJKH".to_string(), quantity: 3 }, RecipeComponent { chemical: "XSGP".to_string(), quantity: 18 }, RecipeComponent { chemical: "CGZL".to_string(), quantity: 22 }, RecipeComponent { chemical: "BNVB".to_string(), quantity: 2 }, RecipeComponent { chemical: "PNSD".to_string(), quantity: 57 }], output: RecipeComponent { chemical: "FUEL".to_string(), quantity: 1 }}
        );
    }

    #[test]
    fn test_cost_for_one_fuel() {
        let recipes = load_recipes("src/inputs/14_sample_1.txt");
        assert_eq!(ore_cost_for_fuel(&recipes, 1), 31);

        let recipes = load_recipes("src/inputs/14_sample_2.txt");
        assert_eq!(ore_cost_for_fuel(&recipes, 1), 13312);

        let recipes = load_recipes("src/inputs/14_sample_3.txt");
        assert_eq!(ore_cost_for_fuel(&recipes, 1), 165);

        let recipes = load_recipes("src/inputs/14_sample_4.txt");
        assert_eq!(ore_cost_for_fuel(&recipes, 1), 180697);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(fourteen_a(), 158482);
    }

    #[test]
    fn test_one_trillion_ore() {
        let recipes = load_recipes("src/inputs/14_sample_2.txt");
        assert_eq!(
            num_fuel_producible_with_one_trillion_ore(&recipes),
            82892753
        );
    }
}
