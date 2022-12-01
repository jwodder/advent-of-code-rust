use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let mut paras = input.paragraphs();
    let rules = paras
        .next()
        .expect("Empty input")
        .lines()
        .map(|s| parse_replacement(s).expect("Parse error"))
        .into_group_map();
    let molecule = split_atoms(paras.next().expect("No molecule in input"));
    let mut outputs = HashSet::new();
    for (i, atom) in molecule.iter().enumerate() {
        if let Some(replacements) = rules.get(atom) {
            for r in replacements {
                let mut newmol = molecule.clone();
                newmol[i] = r.clone();
                outputs.insert(<[String]>::concat(&newmol));
            }
        }
    }
    outputs.len()
}

fn parse_replacement(s: &str) -> Result<(String, String), ParseError> {
    let mut parser = PullParser::new(s);
    let left = parser.parse_to::<String, _>(Token::Whitespace)?;
    parser.skip("=> ")?;
    let right = parser.parse_to::<String, _>(Token::Eof)?;
    Ok((left, right))
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
        let input = Input::from("H => HO\nH => OH\nO => HH\n\nHOH\n");
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn test_example2() {
        let input = Input::from("H => HO\nH => OH\nO => HH\n\nHOHOHO\n");
        assert_eq!(solve(input), 7);
    }
}
