use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap, HashSet};

fn solve(input: Input) -> usize {
    let mut paras = input.paragraphs();
    let rules = paras
        .next()
        .expect("Empty input")
        .lines()
        .map(|s| parse_replacement(s).expect("Parse error"))
        .into_group_map();
    let molecule = split_atoms(paras.next().expect("No molecule in input"));
    let mut current = vec!["e".to_string()];
    let mut visited = HashSet::new();
    let mut distances = HashMap::from([(current.clone(), 0usize)]);
    loop {
        //eprintln!("current = {current:?}");
        for p in apply_replacements(&current, &rules) {
            //eprintln!("  p = {p:?}");
            if p.len() >= molecule.len() && p != molecule {
                //eprintln!("  Discarded");
                continue;
            }
            if !visited.contains(&p) {
                let newdist = distances[&current] + 1;
                //eprintln!("  newdist = {newdist}");
                match distances.entry(p.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(newdist);
                    }
                    Entry::Occupied(mut e) if *e.get() > newdist => {
                        e.insert(newdist);
                    }
                    _ => (),
                };
            }
        }
        visited.insert(current);
        if visited.contains(&molecule) {
            return distances[&molecule];
        }
        current = distances
            .iter()
            .filter(|&(k, _)| !visited.contains(k))
            .min_by_key(|&(_, &dist)| dist)
            .map(|(k, _)| k.clone())
            .expect("No route to endpoint");
    }
}

fn apply_replacements<'a>(
    molecule: &'a [String],
    rules: &'a HashMap<String, Vec<Vec<String>>>,
) -> impl Iterator<Item = Vec<String>> + 'a {
    molecule
        .iter()
        .enumerate()
        .filter_map(move |(i, atom)| {
            Some(rules.get(atom)?.iter().map(move |r| {
                let mut newmol = molecule.to_vec();
                let _ = newmol.splice(i..=i, r.clone());
                newmol
            }))
        })
        .flatten()
}

fn parse_replacement(s: &str) -> Result<(String, Vec<String>), ParseError> {
    let mut parser = PullParser::new(s);
    let left = parser.parse_to::<String, _>(Token::Whitespace)?;
    parser.skip("=> ")?;
    let right = parser.parse_to::<String, _>(Token::Eof)?;
    Ok((left, split_atoms(right)))
}

fn split_atoms(s: String) -> Vec<String> {
    s.chars()
        .peekable()
        .batching(|iter| {
            let mut atom = String::from(iter.next()?);
            atom.extend(iter.take_while_ref(|c| c.is_ascii_lowercase()));
            Some(atom)
        })
        .collect()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_atoms() {
        assert_eq!(
            split_atoms("CRnFYMgAr".into()),
            ["C", "Rn", "F", "Y", "Mg", "Ar"]
        );
    }

    #[test]
    fn test_example1() {
        let input = Input::from("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH\n");
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_example2() {
        let input = Input::from("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO\n");
        assert_eq!(solve(input), 6);
    }
}
