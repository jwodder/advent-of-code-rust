// From drawing the first 200 or so states of the automaton, we see that it
// eventually reaches a cycle-like situation in which each state is just the
// previous shifted one cell to the right.
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{FromBits, Input};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlantAutomaton {
    state: HashSet<i64>,
    rules: HashSet<u8>,
}

impl PlantAutomaton {
    fn advance(&self) -> PlantAutomaton {
        let left = self.state.iter().copied().min().unwrap_or(0) - 2;
        let right = self.state.iter().copied().max().unwrap_or(0) + 2;
        let mut newstate = HashSet::new();
        for i in left..=right {
            let stateslice = u8::from_bits(((i - 2)..=(i + 2)).map(|j| self.state.contains(&j)));
            if self.rules.contains(&stateslice) {
                newstate.insert(i);
            }
        }
        PlantAutomaton {
            state: newstate,
            rules: self.rules.clone(),
        }
    }

    fn is_shifted_right_of(&self, other: &PlantAutomaton) -> bool {
        self.state == other.state.iter().map(|&i| i + 1).collect::<HashSet<_>>()
    }
}

impl std::str::FromStr for PlantAutomaton {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PlantAutomaton, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("initial state: ")?;
        let state = parser
            .scan_to(Token::Whitespace)?
            .chars()
            .zip(0i64..)
            .filter_map(|(c, i)| (c == '#').then_some(i))
            .collect();
        let mut rules = HashSet::new();
        for ln in parser.into_str().lines() {
            let (pre, post) = parse_rule(ln)?;
            if post {
                rules.insert(pre);
            }
        }
        Ok(PlantAutomaton { state, rules })
    }
}

fn parse_rule(s: &str) -> Result<(u8, bool), ParseError> {
    let mut parser = PullParser::new(s);
    let preimg = parser.scan_to(Token::Whitespace)?;
    let pre = u8::from_bits(preimg.chars().map(|c| c == '#'));
    parser.skip("=> ")?;
    let post = parser.parse_to::<char, _>(Token::Eof)? == '#';
    Ok((pre, post))
}

fn solve(input: Input) -> i64 {
    let n = 50_000_000_000;
    let state = input.parse::<PlantAutomaton>();
    for (i, (prev, state)) in std::iter::zip(
        1..n,
        std::iter::successors(Some(state), |st| Some(st.advance())).tuple_windows(),
    ) {
        if state.is_shifted_right_of(&prev) {
            return state.state.into_iter().map(|j| j + n - i).sum();
        }
    }
    panic!("States did not cycle");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
