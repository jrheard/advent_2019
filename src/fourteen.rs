use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, PartialEq)]
struct Node {
    chemical: String,
    quantity: u32,
    children: Vec<Node>,
}

impl Node {
    pub fn new(chemical: String, quantity: u32) -> Node {
        Node {
            chemical,
            quantity,
            children: vec![],
        }
    }
}

struct NodeIntoIter<'a> {
    nodes: VecDeque<&'a Node>,
}

impl<'a> Iterator for NodeIntoIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        match self.nodes.pop_front() {
            Some(node) => {
                self.nodes.extend(node.children.iter());
                Some(node)
            }
            None => None,
        }
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = &'a Node;
    type IntoIter = NodeIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut nodes = VecDeque::new();
        nodes.push_back(self);
        NodeIntoIter { nodes }
    }
}

fn naively_fill_tree(node: &mut Node, recipes: &HashMap<String, Recipe>) {
    if node.chemical == "ORE" {
        return;
    }

    let recipe = &recipes[&node.chemical];
    let desired_output_quantity = node.quantity;
    let required_num_reactions =
        (desired_output_quantity as f32 / recipe.output.quantity as f32).ceil();

    node.children = recipe
        .inputs
        .iter()
        .map(move |input_component| {
            let mut child = Node::new(
                input_component.chemical.clone(),
                input_component.quantity * required_num_reactions as u32,
            );
            naively_fill_tree(&mut child, &recipes);
            child
        })
        .collect();
}

fn total_quantity_of_chemical_in_tree(node: &Node, chemical: &str) -> u32 {
    node.into_iter()
        .filter(|&child| child.chemical == chemical)
        .map(|child| child.quantity)
        .sum()
}

fn find_a_chemical_with_multiple_nodes(
    root: &Node,
    bulk_buy_chemicals: &[String],
) -> Option<String> {
    for chemical in bulk_buy_chemicals {
        if root
            .into_iter()
            .filter(|&node| &node.chemical == chemical)
            .count()
            > 1
        {
            return Some(chemical.clone());
        }
    }

    None
}

fn delete_nodes_with_chemical_from_tree(node: &mut Node, chemical: &str) {
    node.children.retain(|child| child.chemical != chemical);

    for child in &mut node.children {
        delete_nodes_with_chemical_from_tree(child, chemical);
    }
}

/// Returns true if any collapsing happened, false if there was nothing to collapse
fn collapse_bulk_buy_nodes(
    root: &mut Node,
    recipes: &HashMap<String, Recipe>,
    bulk_buy_chemicals: &[String],
) -> bool {
    let chemical_with_multiple_nodes =
        find_a_chemical_with_multiple_nodes(root, bulk_buy_chemicals);

    match chemical_with_multiple_nodes {
        Some(chemical) => {
            let quantity = total_quantity_of_chemical_in_tree(root, &chemical);
            delete_nodes_with_chemical_from_tree(root, &chemical);
            let mut new_node = Node::new(chemical, quantity);
            naively_fill_tree(&mut new_node, recipes);
            root.children.push(new_node);
            true
        }
        None => false,
    }
}

fn cost_for_one_fuel(recipes: &HashMap<String, Recipe>) -> u32 {
    let mut root = Node::new("FUEL".to_string(), 1);
    naively_fill_tree(&mut root, recipes);
    dbg!(&root);

    let bulk_buy_chemicals: Vec<String> = recipes
        .values()
        .filter_map(|recipe| {
            if recipe.output.quantity > 1 {
                Some(recipe.output.chemical.clone())
            } else {
                None
            }
        })
        .collect();

    while collapse_bulk_buy_nodes(&mut root, &recipes, &bulk_buy_chemicals) {}

    total_quantity_of_chemical_in_tree(&root, "ORE")
}

// TODO see if i can remove this step once i get 14a passing
/// TODO oh my god rename and document
fn perform_simple_reductions(
    components: &[RecipeComponent],
    recipes: &HashMap<String, Recipe>,
) -> Vec<RecipeComponent> {
    components
        .iter()
        .flat_map(|component| {
            if component.chemical == "ORE" {
                // Ore's our base material, there isn't a recipe for it.
                return vec![component.clone()];
            }

            let recipe = &recipes[&component.chemical];

            if recipe.output.quantity == 1 {
                if recipe.inputs.len() == 1 && recipe.inputs[0].chemical == "ORE" {
                    // We've bottomed out for this particular component for now!
                    vec![component.clone()]
                } else {
                    // Substitute `recipe` out for its constituent parts.
                    perform_simple_reductions(&recipe.inputs, recipes)
                        .iter()
                        .map(|reduction_component| RecipeComponent {
                            chemical: reduction_component.chemical.clone(),
                            quantity: reduction_component.quantity * component.quantity,
                        })
                        .collect()
                }
            } else {
                vec![component.clone()]
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

/// TODO oh my god rename and document
fn component_costs(
    components: &[RecipeComponent],
    recipes: &HashMap<String, Recipe>,
) -> Vec<RecipeComponent> {
    components
        .iter()
        .flat_map(|component| {
            if component.chemical == "ORE" {
                return vec![component.clone()];
            }

            let recipe = &recipes[&component.chemical];

            let desired_output_quantity = component.quantity;
            let required_num_reactions =
                (desired_output_quantity as f32 / recipe.output.quantity as f32).ceil();

            recipe
                .inputs
                .iter()
                .map(move |input_component| RecipeComponent {
                    chemical: input_component.chemical.clone(),
                    quantity: input_component.quantity * required_num_reactions as u32,
                })
                .collect()
        })
        .collect()

    // TODO i think it might be a good idea to record a history of simplified things
    // i can't tell if that's something that would make sense to do here, i.e. to have this
    // return  a 2-tuple of (current_return_value, something_else)
    // or if it's something that should happen in the loop, between step 4 and step 1
    // figure it out tomorrow!!!!
}

fn cost_for_one_fuel_old(recipes: &HashMap<String, Recipe>) -> u32 {
    let mut bulk_buys: HashMap<String, Vec<u32>> = HashMap::new();
    let mut components = recipes["FUEL"].inputs.clone();
    //println!("1: {:?}", components);

    while components.len() > 1 {
        components = perform_simple_reductions(&components, &recipes);
        //println!("2: {:?}", components);
        components = merge_components(&components);
        //println!("3: {:?}", components);

        for component in components.clone() {
            bulk_buys
                .entry(component.chemical)
                .or_insert_with(Vec::new)
                .push(component.quantity);
        }

        components = component_costs(&components, &recipes);
        //println!("4: {:?}", components);
        components = merge_components(&components);
        //println!("5: {:?}", components);
    }

    dbg!(bulk_buys);

    println!("6: {:?}", components);
    components[0].quantity
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
            perform_simple_reductions(
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
        let required = perform_simple_reductions(
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
        //let recipes = load_recipes("src/inputs/14_sample_1.txt");
        //assert_eq!(cost_for_one_fuel(&recipes), 31);

        //let recipes = load_recipes("src/inputs/14_sample_3.txt");
        //assert_eq!(cost_for_one_fuel(&recipes), 165);

        let recipes = load_recipes("src/inputs/14_sample_2.txt");
        assert_eq!(cost_for_one_fuel(&recipes), 13312);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(fourteen_a(), 0);
    }

    #[test]
    fn test_tree_iteration() {
        let mut root = Node::new("FOO".to_string(), 5);
        root.children.push(Node::new("BAR".to_string(), 10));
        root.children.push(Node::new("BAZ".to_string(), 1));
        root.children[1]
            .children
            .push(Node::new("QUUX".to_string(), 100));

        let vector: Vec<&Node> = root.into_iter().collect();

        assert_eq!(
            vector,
            vec![
                &root,
                &root.children[0],
                &root.children[1],
                &root.children[1].children[0]
            ]
        );
    }
}
