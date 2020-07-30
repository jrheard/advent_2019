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

#[derive(Debug, PartialEq)]
struct Node {
    chemical: String,
    quantity: u64,
    children: Vec<Node>,
}

impl Node {
    pub fn new(chemical: String, quantity: u64) -> Node {
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

/// Recursively expands the tree in `node` by following the recipes in `recipes`, bottoming out at ORE.
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
                input_component.quantity * required_num_reactions as u64,
            );
            naively_fill_tree(&mut child, &recipes);
            child
        })
        .collect();
}

/// Returns the total `quantity` of `chemical` in the tree represented by `node`.
fn total_quantity_of_chemical_in_tree(node: &Node, chemical: &str) -> u64 {
    node.into_iter()
        .filter(|&child| child.chemical == chemical)
        .map(|child| child.quantity)
        .sum()
}

/// Returns Some(chemical) if there's a chemical in `root` that appears in multiple nodes, None otherwise.
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

/// Removes all Nodes with `chemical` from the tree represented by `node`.
fn delete_nodes_with_chemical_from_tree(node: &mut Node, chemical: &str) {
    node.children.retain(|child| child.chemical != chemical);

    for child in &mut node.children {
        delete_nodes_with_chemical_from_tree(child, chemical);
    }
}

/// Searches the tree in `root` for a chemical in `bulk_buy_chemicals` that appears in multiple Nodes.
/// If a chemical is found, all Nodes with that chemical are collapsed together into a single Node.
/// Returns true if any collapsing happened, false if there was nothing to collapse.
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

/// Returns the lowest depth at which `chemical` was found in the tree represented by `node`.
fn lowest_depth_seen(node: &Node, chemical: &str, depth: u64) -> Option<u64> {
    if node.chemical == chemical {
        Some(depth)
    } else if node.children.is_empty() {
        None
    } else {
        node.children
            .iter()
            .map(|child| lowest_depth_seen(child, chemical, depth + 1))
            .max()?
    }
}

/// Returns the minimum amount of ORE required to produce exactly 1 FUEL according to `recipes`.
fn cost_for_fuel_amount(recipes: &HashMap<String, Recipe>, quantity: u64) -> u64 {
    let mut root = Node::new("FUEL".to_string(), quantity);
    naively_fill_tree(&mut root, recipes);

    let mut bulk_buy_chemicals: Vec<String> = recipes
        .values()
        .filter_map(|recipe| {
            if recipe.output.quantity > 1 {
                Some(recipe.output.chemical.clone())
            } else {
                None
            }
        })
        .collect();

    bulk_buy_chemicals.sort_by_key(|chemical| lowest_depth_seen(&root, chemical, 0));

    while collapse_bulk_buy_nodes(&mut root, &recipes, &bulk_buy_chemicals) {}

    total_quantity_of_chemical_in_tree(&root, "ORE")
}

pub fn fourteen_a() -> u64 {
    let recipes = load_recipes("src/inputs/14.txt");
    cost_for_fuel_amount(&recipes, 1)
}

struct Nanofactory<'a> {
    chemical_amounts: HashMap<&'a str, u64>,
    ore_spent: u64,
}

impl<'a> Nanofactory<'a> {
    fn perform_recipe(
        &mut self,
        chemical: &'a str,
        quantity: u64,
        recipes: &'a HashMap<String, Recipe>,
    ) {
        if chemical == "ORE" {
            self.ore_spent += quantity;
            return;
        }

        let recipe = &recipes[chemical];

        for component in &recipe.inputs {
            self.perform_recipe(&component.chemical, component.quantity as u64, recipes);

            if chemical != "ORE" {
                self.chemical_amounts
                    .entry(&component.chemical)
                    .and_modify(|amount| *amount -= component.quantity as u64);
            }
        }

        self.chemical_amounts
            .entry(&chemical)
            .and_modify(|amount| *amount += quantity);
    }

    pub fn new(recipes: &'a HashMap<String, Recipe>) -> Self {
        let chemical_amounts: HashMap<&str, u64> = recipes
            .keys()
            .map(|chemical| (chemical.as_str(), 0))
            .collect();

        Nanofactory {
            chemical_amounts,
            ore_spent: 0,
        }
    }
}

fn num_fuel_producible_with_one_trillion_ore_old(recipes: &HashMap<String, Recipe>) -> u64 {
    let mut chemical_amounts: HashMap<&str, u64> = recipes
        .keys()
        .map(|chemical| (chemical.as_str(), 0))
        .collect();

    chemical_amounts.insert("ORE", ONE_TRILLION as u64);

    //let mut factory = Nanofactory {
    //chemical_amounts,
    //fuel_produced: 0,
    //};
    //
    //while factory.produce_one_fuel(&recipes) {}

    //factory.fuel_produced
    5
}

fn ore_cost_for_fuel(recipes: &HashMap<String, Recipe>, fuel_quantity: u64) -> u64 {
    let mut factory = Nanofactory::new(recipes);
    factory.perform_recipe("FUEL", fuel_quantity, recipes);
    factory.ore_spent
}

fn num_fuel_producible_with_one_trillion_ore(recipes: &HashMap<String, Recipe>) -> u64 {
    let mut lower_bound = ONE_TRILLION / cost_for_fuel_amount(&recipes, 1);
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
        assert_eq!(cost_for_fuel_amount(&recipes, 1), 31);

        let recipes = load_recipes("src/inputs/14_sample_2.txt");
        assert_eq!(cost_for_fuel_amount(&recipes, 1), 13312);

        let recipes = load_recipes("src/inputs/14_sample_3.txt");
        assert_eq!(cost_for_fuel_amount(&recipes, 1), 165);

        let recipes = load_recipes("src/inputs/14_sample_4.txt");
        assert_eq!(cost_for_fuel_amount(&recipes, 1), 180697);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(fourteen_a(), 158482);
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

    #[test]
    fn test_one_trillion_ore() {
        let recipes = load_recipes("src/inputs/14_sample_2.txt");
        assert_eq!(
            num_fuel_producible_with_one_trillion_ore(&recipes),
            82892753
        );
    }
}
