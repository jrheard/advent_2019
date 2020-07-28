use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

static OUTER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) => (.*)").unwrap());
static COMPONENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]*) ([A-Z]*)").unwrap());

#[derive(PartialEq, Eq, Debug, Hash)]
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

fn parse_recipe(recipe: &str) -> (Vec<RecipeComponent>, RecipeComponent) {
    let captures = OUTER_RE.captures(recipe).unwrap();
    let inputs = captures[1].split(", ").map(RecipeComponent::new).collect();

    (inputs, RecipeComponent::new(&captures[2]))
}

pub fn fourteen_a() -> u32 {
    let _recipes = load_recipes();

    5
}

fn load_recipes() -> HashMap<RecipeComponent, Vec<RecipeComponent>> {
    let contents = fs::read_to_string("src/inputs/14.txt").unwrap();
    contents
        .lines()
        .map(parse_recipe)
        .map(|(inputs, output)| (output, inputs))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_recipe() {
        assert_eq!(
            parse_recipe("7 LCSV, 1 LKPNB, 36 CMNH, 1 JZXPH, 20 DGJPN, 3 WDWB, 69 DXJKC, 3 WHJKH, 18 XSGP, 22 CGZL, 2 BNVB, 57 PNSD => 1 FUEL"),
            (vec![RecipeComponent { chemical: "LCSV".to_string(), quantity: 7 }, RecipeComponent { chemical: "LKPNB".to_string(), quantity: 1 }, RecipeComponent { chemical: "CMNH".to_string(), quantity: 36 }, RecipeComponent { chemical: "JZXPH".to_string(), quantity: 1 }, RecipeComponent { chemical: "DGJPN".to_string(), quantity: 20 }, RecipeComponent { chemical: "WDWB".to_string(), quantity: 3 }, RecipeComponent { chemical: "DXJKC".to_string(), quantity: 69 }, RecipeComponent { chemical: "WHJKH".to_string(), quantity: 3 }, RecipeComponent { chemical: "XSGP".to_string(), quantity: 18 }, RecipeComponent { chemical: "CGZL".to_string(), quantity: 22 }, RecipeComponent { chemical: "BNVB".to_string(), quantity: 2 }, RecipeComponent { chemical: "PNSD".to_string(), quantity: 57 }], RecipeComponent { chemical: "FUEL".to_string(), quantity: 1 })
        );
    }
}
