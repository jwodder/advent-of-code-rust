use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

struct Distance {
    point_a: String,
    point_b: String,
    dist: usize,
}

impl FromStr for Distance {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Distance, ParseError> {
        let mut parser = PullParser::new(s);
        let point_a = parser.parse_to::<String, _>(Token::Whitespace)?;
        parser.skip("to ")?;
        let point_b = parser.parse_to::<String, _>(Token::Whitespace)?;
        parser.skip("= ")?;
        let dist = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Distance {
            point_a,
            point_b,
            dist,
        })
    }
}

fn travelling_santa<I: IntoIterator<Item = Distance>>(iter: I) -> usize {
    let mut distances = HashMap::new();
    let mut point2id = HashMap::new();
    for d in iter {
        let qty = point2id.len();
        let p1 = *point2id.entry(d.point_a).or_insert(qty);
        let qty = point2id.len();
        let p2 = *point2id.entry(d.point_b).or_insert(qty);
        distances.insert((p1, p2), d.dist);
        distances.insert((p2, p1), d.dist);
    }
    let qty = point2id.len();
    (0..qty)
        .permutations(qty)
        .map(|perm| {
            perm.windows(2)
                .map(|w| distances[&(w[0], w[1])])
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

fn main() {
    println!(
        "{}",
        travelling_santa(Input::from_env().parse_lines::<Distance>())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let distances = [
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ]
        .into_iter()
        .map(|s| s.parse::<Distance>().unwrap());
        assert_eq!(travelling_santa(distances), 605);
    }
}
