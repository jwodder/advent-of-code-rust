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
    let mut max_steps = 1;
    for n in nodes.keys() {
        if n.ends_with('A') {
            let mut n = n.clone();
            let mut steps = 0;
            for d in directions.trim().chars().cycle() {
                n = match d {
                    'L' => nodes[&n].0.clone(),
                    'R' => nodes[&n].1.clone(),
                    c => panic!("Unexpected direction: {c:?}"),
                };
                steps += 1;
                if n.ends_with('Z') {
                    break;
                }
            }
            max_steps = lcm(max_steps, steps);
        }
    }
    max_steps
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(x: usize, y: usize) -> usize {
    let d = gcd(x, y);
    if d == 0 { 0 } else { x * y / d }
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
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        ));
        assert_eq!(solve(input), 6);
    }
}
