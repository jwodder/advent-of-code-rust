use adventutil::index::Index;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;

fn parse_orbit(s: &str) -> Result<(String, String), ParseError> {
    let mut parser = PullParser::new(s);
    let center = parser.parse_to::<String, _>(')')?;
    let in_orbit = parser.parse_to::<String, _>(Token::Eof)?;
    Ok((center, in_orbit))
}

struct DepthLookup {
    outer2inner: HashMap<usize, usize>,
    cache: HashMap<usize, u32>,
}

impl DepthLookup {
    fn new(outer2inner: HashMap<usize, usize>) -> DepthLookup {
        DepthLookup {
            outer2inner,
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, i: usize) -> u32 {
        if let Some(&j) = self.cache.get(&i) {
            j
        } else {
            let depth = match self.outer2inner.get(&i) {
                Some(j) => 1 + self.get(*j),
                None => 0,
            };
            self.cache.insert(i, depth);
            depth
        }
    }
}

fn solve(input: Input) -> u32 {
    let mut index = Index::new();
    let mut outer2inner = HashMap::new();
    for (center, in_orbit) in input.lines().map(|s| parse_orbit(&s).expect("Parse error")) {
        let ci = index.get(center);
        let oi = index.get(in_orbit);
        outer2inner.insert(oi, ci);
    }
    let mut depths = DepthLookup::new(outer2inner);
    index.into_indices().map(|i| depths.get(i)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "COM)B\n", "B)C\n", "C)D\n", "D)E\n", "E)F\n", "B)G\n", "G)H\n", "D)I\n", "E)J\n",
            "J)K\n", "K)L\n",
        ));
        assert_eq!(solve(input), 42);
    }
}
