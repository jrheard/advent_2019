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

fn ore_cost_for_fuel(recipes: &HashMap<String, Recipe>, fuel_quantity: u64) -> u64 {
    let mut shopping_cart: VecDeque<RecipeComponent> = VecDeque::new();
    shopping_cart.push_back(RecipeComponent {
        chemical: "FUEL".to_string(),
        quantity: fuel_quantity,
    });

    let mut chemical_bank: HashMap<String, u64> = HashMap::new();

    let mut ore_spent = 0;

    while !shopping_cart.is_empty() {
        let component = shopping_cart.pop_front().unwrap();

        if component.chemical == "ORE" {
            ore_spent += component.quantity;
            continue;
        }

        let recipe = &recipes[&component.chemical];
        let desired_output_quantity = component.quantity;

        let bank_entry = chemical_bank.entry(component.chemical).or_insert(0);
        if *bank_entry >= desired_output_quantity {
            // We have enough of that chemical lying around already.
            *bank_entry -= desired_output_quantity;
        } else {
            // We don't have enough of that chemical stored, let's make some.

            let missing_amount = desired_output_quantity - *bank_entry;

            let required_num_reactions =
                (missing_amount as f64 / recipe.output.quantity as f64).ceil() as u64;

            for input in &recipe.inputs {
                shopping_cart.push_back(RecipeComponent {
                    chemical: input.chemical.clone(),
                    quantity: input.quantity * required_num_reactions,
                });
            }

            *bank_entry += recipe.output.quantity * required_num_reactions;
            *bank_entry -= desired_output_quantity;
        }
    }

    ore_spent
}

pub fn fourteen_a() -> u64 {
    let recipes = load_recipes("src/inputs/14.txt");
    ore_cost_for_fuel(&recipes, 1)
}

fn num_fuel_producible_with_one_trillion_ore(recipes: &HashMap<String, Recipe>) -> u64 {
    let mut lower_bound = ONE_TRILLION / ore_cost_for_fuel(&recipes, 1);
    let mut upper_bound = 10 * lower_bound;

    while ore_cost_for_fuel(&recipes, upper_bound) < ONE_TRILLION {
        lower_bound = upper_bound;
        upper_bound *= 10;
    }

    loop {
        let midpoint = (lower_bound + upper_bound) / 2;
        let cost = ore_cost_for_fuel(&recipes, midpoint);

        if cost <= ONE_TRILLION && ore_cost_for_fuel(&recipes, midpoint + 1) > ONE_TRILLION {
            return midpoint;
        }

        if cost < ONE_TRILLION {
            lower_bound = midpoint;
        } else {
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
        assert_eq!(fourteen_b(), 7993831);
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
