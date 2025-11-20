use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet, hash_map::Entry};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl std::str::FromStr for Food {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Food, ParseError> {
        let mut parser = PullParser::new(s);
        let ingredients = parser
            .scan_to("(contains ")?
            .split_ascii_whitespace()
            .map(ToOwned::to_owned)
            .collect();
        let allergens = parser
            .scan_to(')')?
            .split(',')
            .map(|s| s.trim().to_owned())
            .collect();
        parser.eof()?;
        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

fn solve(input: Input) -> String {
    let foods = input.parse_lines::<Food>().collect::<Vec<_>>();
    // Mapping from allergens to ingredients that may contain them:
    let mut candidates = HashMap::<String, HashSet<String>>::new();
    for f in &foods {
        for allergen in &f.allergens {
            match candidates.entry(allergen.clone()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().retain(|a| f.ingredients.contains(a));
                }
                Entry::Vacant(e) => {
                    e.insert(f.ingredients.clone());
                }
            }
        }
    }
    let mut assignments = BTreeMap::new();
    while !candidates.is_empty() {
        let Some((allergen, ingreds)) = candidates.iter().find(|(_, ingreds)| ingreds.len() == 1)
        else {
            panic!("No singleton possibility sets left!");
        };
        let allergen = allergen.clone();
        let ingredient = ingreds.iter().next().unwrap().clone();
        candidates.remove(&allergen);
        for ingreds in candidates.values_mut() {
            ingreds.remove(&ingredient);
        }
        assignments.insert(allergen, ingredient);
    }
    assignments.into_values().join(",")
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)\n",
            "sqjhc fvjkl (contains soy)\n",
            "sqjhc mxmxvkd sbzzf (contains fish)\n",
        ));
        assert_eq!(solve(input), "mxmxvkd,sqjhc,fvjkl");
    }
}
