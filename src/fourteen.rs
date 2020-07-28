use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

static OUTER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) => (.*)").unwrap());
static COMPONENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]*) ([A-Z]*)").unwrap());

#[derive(PartialEq, Debug)]
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
    quantity: u32,
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

fn required_chemicals_for(
    target: &[RecipeComponent],
    recipes: &HashMap<String, Recipe>,
) -> Vec<RecipeComponent> {
    target
        .iter()
        .flat_map(|component| {
            let inputs = &recipes[&component.chemical].inputs;

            if inputs.len() == 1 && inputs[0].chemical == "ORE" {
                vec![component.clone()]
            } else {
                required_chemicals_for(&inputs, recipes)
            }
        })
        .collect()
}

fn merge_components(components: &[RecipeComponent]) -> Vec<RecipeComponent> {
    let grouped_components = components
        .iter()
        .map(|component| (&component.chemical, component))
        .into_group_map();

    grouped_components
        .iter()
        .map(|(chemical, component_group)| RecipeComponent {
            chemical: (*chemical).clone(),
            quantity: component_group
                .iter()
                .map(|component| component.quantity)
                .sum(),
        })
        .collect()
}

fn component_costs_in_ore(
    components: &[RecipeComponent],
    recipes: &HashMap<String, Recipe>,
) -> u32 {
    components
        .iter()
        .map(|component| {
            let recipe = &recipes[&component.chemical];
            assert_eq!(recipe.inputs.len(), 1);
            assert_eq!(&recipe.inputs[0].chemical, "ORE");

            let ore_per_reaction = recipe.inputs[0].quantity;
            let desired_output_quantity = component.quantity;
            let required_num_reactions =
                (desired_output_quantity as f32 / recipe.output.quantity as f32).ceil();

            required_num_reactions as u32 * ore_per_reaction
        })
        .sum()
}

fn cost_for_one_fuel(recipes: &HashMap<String, Recipe>) -> u32 {
    let leaf_costs = required_chemicals_for(
        &[RecipeComponent {
            chemical: "FUEL".to_string(),
            quantity: 1,
        }],
        &recipes,
    );

    let merged_leaf_costs = merge_components(&leaf_costs);

    component_costs_in_ore(&merged_leaf_costs, recipes)
}

pub fn fourteen_a() -> u32 {
    let recipes = load_recipes("src/inputs/14.txt");
    cost_for_one_fuel(&recipes)
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
    fn test_required_chemicals_for() {
        let recipes = load_recipes("src/inputs/14_sample_1.txt");

        assert_eq!(
            required_chemicals_for(
                &[RecipeComponent {
                    chemical: "FUEL".to_string(),
                    quantity: 1
                }],
                &recipes
            ),
            vec![
                RecipeComponent {
                    chemical: "A".to_string(),
                    quantity: 7
                },
                RecipeComponent {
                    chemical: "A".to_string(),
                    quantity: 7
                },
                RecipeComponent {
                    chemical: "A".to_string(),
                    quantity: 7
                },
                RecipeComponent {
                    chemical: "A".to_string(),
                    quantity: 7
                },
                RecipeComponent {
                    chemical: "B".to_string(),
                    quantity: 1
                }
            ]
        );
    }

    #[test]
    fn test_merge_components() {
        let recipes = load_recipes("src/inputs/14_sample_1.txt");
        let required = required_chemicals_for(
            &[RecipeComponent {
                chemical: "FUEL".to_string(),
                quantity: 1,
            }],
            &recipes,
        );

        let mut merged = merge_components(&required);
        merged.sort_by_key(|component| component.chemical.to_string());

        assert_eq!(
            merged,
            vec![
                RecipeComponent {
                    chemical: "A".to_string(),
                    quantity: 28
                },
                RecipeComponent {
                    chemical: "B".to_string(),
                    quantity: 1
                },
            ]
        );
    }

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
        assert_eq!(cost_for_one_fuel(&recipes), 31);

        let recipes = load_recipes("src/inputs/14_sample_2.txt");
        assert_eq!(cost_for_one_fuel(&recipes), 13312);
    }
}
