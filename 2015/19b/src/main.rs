use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{dijkstra_length, Input};
use itertools::Itertools;
use std::collections::HashMap;

fn solve(input: Input) -> u32 {
    let (rules, molecule) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input was not exactly two paragraphs");
    let rules = rules
        .lines()
        .map(|s| {
            let (before, after) = parse_replacement(s).expect("Parse error");
            (after, before)
        })
        .collect::<HashMap<String, String>>();
    dijkstra_length(molecule, "e".to_string(), |current| {
        apply_replacements(current, &rules)
    })
    .unwrap()
}

fn apply_replacements(
    molecule: &str,
    rules: &HashMap<String, String>,
) -> impl Iterator<Item = (String, u32)> {
    let mut modified = Vec::new();
    for (after, before) in rules {
        let mut i = 0;
        while let Some(n) = molecule[i..].find(after) {
            let mut s = molecule[..n].to_string();
            s.push_str(before);
            s.push_str(&molecule[(n + after.len())..]);
            modified.push(s);
            i += n + 1;
        }
    }
    modified.into_iter().map(|s| (s, 1))
}

fn parse_replacement(s: &str) -> Result<(String, String), ParseError> {
    let mut parser = PullParser::new(s);
    let left = parser.parse_to::<String, _>(Token::Whitespace)?;
    parser.skip("=> ")?;
    let right = parser.parse_to::<String, _>(Token::Eof)?;
    Ok((left, right))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

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
