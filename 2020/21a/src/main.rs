use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser};
use std::collections::{HashMap, HashSet, hash_map::Entry};

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

fn solve(input: Input) -> usize {
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
    let allergenic = candidates.into_values().flatten().collect::<HashSet<_>>();
    let mut qty = 0;
    for f in &foods {
        qty += f.ingredients.difference(&allergenic).count();
    }
    qty
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
        assert_eq!(solve(input), 5);
    }
}
