use adventutil::Input;
use adventutil::index::Index;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Preference {
    person: String,
    neighbor: String,
    happiness: i32,
}

impl std::str::FromStr for Preference {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Preference, ParseError> {
        let mut parser = PullParser::new(s);
        let person = parser.parse_to::<String, _>(Token::Whitespace)?;
        parser.skip("would ")?;
        let sign = match parser.scan_to(Token::Whitespace)? {
            "gain" => 1,
            "lose" => -1,
            other => return Err(ParseError::InvalidToken(other.into())),
        };
        let happiness = sign * parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("happiness units by sitting next to ")?;
        let neighbor = parser.parse_to::<String, _>('.')?;
        parser.eof()?;
        Ok(Preference {
            person,
            neighbor,
            happiness,
        })
    }
}

fn solve(input: Input) -> i32 {
    let mut happinesses = HashMap::new();
    let mut person2id = Index::new();
    for p in input.parse_lines::<Preference>() {
        let p1 = person2id.get(p.person);
        let p2 = person2id.get(p.neighbor);
        happinesses.insert((p1, p2), p.happiness);
    }
    let qty = person2id.len();
    (0..qty)
        .permutations(qty)
        .map(|perm| {
            score_with_zero(
                perm.into_iter()
                    .circular_tuple_windows()
                    .map(|(i, j)| happinesses[&(i, j)] + happinesses[&(j, i)]),
            )
        })
        .max()
        .unwrap()
}

fn score_with_zero<I: IntoIterator<Item = i32>>(iter: I) -> i32 {
    let scores = iter.into_iter().collect::<Vec<_>>();
    let worst = *scores.iter().min().unwrap();
    scores.into_iter().sum::<i32>() - worst
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
