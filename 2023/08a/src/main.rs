use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl std::str::FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Node, ParseError> {
        let mut parser = PullParser::new(s);
        let name = parser.scan_to(Token::Whitespace)?.to_owned();
        parser.skip('=')?;
        parser.skip(Token::Whitespace)?;
        parser.skip('(')?;
        let left = parser.scan_to(',')?.to_owned();
        parser.skip(Token::Whitespace)?;
        let right = parser.scan_to(')')?.to_owned();
        parser.eof()?;
        Ok(Node { name, left, right })
    }
}

fn solve(input: Input) -> usize {
    let (directions, node_lines) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let mut nodes = std::collections::HashMap::new();
    for ln in node_lines.lines() {
        let Node { name, left, right } = ln.parse::<Node>().unwrap();
        nodes.insert(name, (left, right));
    }
    let mut loc = String::from("AAA");
    let mut steps = 0;
    for d in directions.trim().chars().cycle() {
        loc = match d {
            'L' => nodes[&loc].0.clone(),
            'R' => nodes[&loc].1.clone(),
            c => panic!("Unexpected direction: {c:?}"),
        };
        steps += 1;
        if loc == "ZZZ" {
            return steps;
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "RL\n",
            "\n",
            "AAA = (BBB, CCC)\n",
            "BBB = (DDD, EEE)\n",
            "CCC = (ZZZ, GGG)\n",
            "DDD = (DDD, DDD)\n",
            "EEE = (EEE, EEE)\n",
            "GGG = (GGG, GGG)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        ));
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "LLR\n",
            "\n",
            "AAA = (BBB, BBB)\n",
            "BBB = (AAA, ZZZ)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        ));
        assert_eq!(solve(input), 6);
    }
}
