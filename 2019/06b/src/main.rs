use adventutil::index::Index;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{dijkstra_length, Input};
use std::collections::HashMap;

fn parse_orbit(s: &str) -> Result<(String, String), ParseError> {
    let mut parser = PullParser::new(s);
    let center = parser.parse_to::<String, _>(')')?;
    let in_orbit = parser.parse_to::<String, _>(Token::Eof)?;
    Ok((center, in_orbit))
}

fn solve(input: Input) -> u32 {
    let mut index = Index::new();
    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    for (center, in_orbit) in input.lines().map(|s| parse_orbit(&s).expect("Parse error")) {
        let ci = index.get(center);
        let oi = index.get(in_orbit);
        connections.entry(ci).or_default().push(oi);
        connections.entry(oi).or_default().push(ci);
    }
    let start = index.get("YOU".into());
    let end = index.get("SAN".into());
    dijkstra_length(start, end, |i| connections[i].iter().map(|&j| (j, 1))).unwrap() - 2
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
            "J)K\n", "K)L\n", "K)YOU\n", "I)SAN\n",
        ));
        assert_eq!(solve(input), 4);
    }
}
