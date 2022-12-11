use adventutil::index::Index;
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

fn solve(input: Input) -> usize {
    let mut distances = HashMap::new();
    let mut point2id = Index::new();
    for d in input.parse_lines::<Distance>() {
        let p1 = point2id.get(d.point_a);
        let p2 = point2id.get(d.point_b);
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
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "London to Dublin = 464\n",
            "London to Belfast = 518\n",
            "Dublin to Belfast = 141\n",
        ));
        assert_eq!(solve(input), 982);
    }
}
