use adventutil::Input;
use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Int(left), Int(right)) => left.cmp(right),
            (List(left), List(right)) => left.cmp(right),
            (left @ Int(_), right @ List(_)) => List(vec![left.clone()]).cmp(right),
            (left @ List(_), right @ Int(_)) => left.cmp(&List(vec![right.clone()])),
        }
    }
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Packet, ParseError> {
        let mut s = s.strip_prefix('[').ok_or(ParseError::BadStart)?;
        let mut stack = vec![Vec::new()];
        loop {
            if let Some(t) = s.strip_prefix(']') {
                let list = Packet::List(stack.pop().unwrap());
                if stack.is_empty() {
                    if t.is_empty() {
                        return Ok(list);
                    } else {
                        return Err(ParseError::TrailingCharacters(s.into()));
                    }
                } else {
                    stack.last_mut().unwrap().push(list);
                    s = t.trim_start_matches(',');
                }
            } else if let Some(t) = s.strip_prefix('[') {
                stack.push(Vec::new());
                s = t;
            } else {
                let i = s.find([',', ']']).unwrap();
                let n = s[..i].parse::<u32>()?;
                stack.last_mut().unwrap().push(Packet::Int(n));
                s = s[i..].trim_start_matches(',');
            }
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("packet does not start with '['")]
    BadStart,
    #[error("invalid integer: {0}")]
    InvalidInt(#[from] ParseIntError),
    #[error("packet has trailing characters: {0:?}")]
    TrailingCharacters(String),
}

fn solve(input: Input) -> usize {
    use Packet::*;
    let divider1 = List(vec![List(vec![Int(2)])]);
    let divider2 = List(vec![List(vec![Int(6)])]);
    let mut packets = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Packet>().expect("ParseError"))
        .collect::<Vec<_>>();
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let i1 = packets.binary_search(&divider1).unwrap() + 1;
    let i2 = packets.binary_search(&divider2).unwrap() + 1;
    i1 * i2
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use Packet::*;

    #[rstest]
    #[case(
        List(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]),
        List(vec![Int(1), Int(1), Int(5), Int(1), Int(1)]),
        Ordering::Less,
    )]
    #[case(
        List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])]),
        List(vec![List(vec![Int(1)]), Int(4)]),
        Ordering::Less,
    )]
    #[case(
        List(vec![Int(9)]),
        List(vec![List(vec![Int(8), Int(7), Int(6)])]),
        Ordering::Greater,
    )]
    #[case(
        List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4)]),
        List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4), Int(4)]),
        Ordering::Less,
    )]
    #[case(
        List(vec![Int(7), Int(7), Int(7), Int(7)]),
        List(vec![Int(7), Int(7), Int(7)]),
        Ordering::Greater,
    )]
    #[case(List(Vec::new()), List(vec![Int(3)]), Ordering::Less)]
    #[case(
        List(vec![List(vec![List(vec![])])]),
        List(vec![List(vec![])]),
        Ordering::Greater,
    )]
    #[case(
        List(vec![Int(1), List(vec![Int(2), List(vec![Int(3), List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])])])]), Int(8), Int(9)]),
        List(vec![Int(1), List(vec![Int(2), List(vec![Int(3), List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])])])]), Int(8), Int(9)]),
        Ordering::Greater,
    )]
    fn test_ord_packet(#[case] left: Packet, #[case] right: Packet, #[case] cmp: Ordering) {
        assert_eq!(left.cmp(&right), cmp);
    }

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "[1,1,3,1,1]\n",
            "[1,1,5,1,1]\n",
            "\n",
            "[[1],[2,3,4]]\n",
            "[[1],4]\n",
            "\n",
            "[9]\n",
            "[[8,7,6]]\n",
            "\n",
            "[[4,4],4,4]\n",
            "[[4,4],4,4,4]\n",
            "\n",
            "[7,7,7,7]\n",
            "[7,7,7]\n",
            "\n",
            "[]\n",
            "[3]\n",
            "\n",
            "[[[]]]\n",
            "[[]]\n",
            "\n",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
        ));
        assert_eq!(solve(input), 140);
    }
}
