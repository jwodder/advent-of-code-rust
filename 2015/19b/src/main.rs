// Based on <https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4k8ca/>.
// Note that this strategy does not work for the examples in the problem
// description.
use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
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
            (revstr(&after), revstr(&before))
        })
        .collect::<HashMap<String, String>>();
    let mut molecule = revstr(&molecule);
    let mut replacements = 0;
    while molecule != "e" {
        let (i, after, before) = rules
            .iter()
            .filter_map(|(after, before)| {
                let i = molecule.find(after)?;
                Some((i, after, before))
            })
            .min_by_key(|tup| tup.0)
            .unwrap();
        molecule.replace_range(i..(i + after.len()), before);
        replacements += 1;
    }
    replacements
}

fn revstr(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.reverse();
    chars.into_iter().collect()
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
