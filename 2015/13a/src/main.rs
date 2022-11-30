use adventutil::index::Index;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

struct Preference {
    person: String,
    neighbor: String,
    happiness: i32,
}

impl FromStr for Preference {
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

fn arrange_seats<I: IntoIterator<Item = Preference>>(iter: I) -> i32 {
    let mut happinesses = HashMap::new();
    let mut person2id = Index::new();
    for p in iter {
        let p1 = person2id.get(p.person);
        let p2 = person2id.get(p.neighbor);
        happinesses.insert((p1, p2), p.happiness);
    }
    let qty = person2id.len();
    (0..qty)
        .permutations(qty)
        .map(|perm| {
            perm.into_iter()
                .circular_tuple_windows()
                .map(|(i, j)| happinesses[&(i, j)] + happinesses[&(j, i)])
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn main() {
    println!(
        "{}",
        arrange_seats(Input::from_env().parse_lines::<Preference>())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let prefs = [
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ]
        .into_iter()
        .map(|s| s.parse::<Preference>().unwrap());
        assert_eq!(arrange_seats(prefs), 330);
    }
}
