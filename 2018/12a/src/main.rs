use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{FromBits, Input};
use std::collections::HashSet;
use std::str::FromStr;

struct PlantAutomaton {
    state: HashSet<isize>,
    rules: HashSet<u8>,
}

impl PlantAutomaton {
    fn advance(self) -> PlantAutomaton {
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
            rules: self.rules,
        }
    }

    fn gen20_sum(mut self) -> isize {
        for _ in 0..20 {
            self = self.advance();
        }
        self.state.into_iter().sum()
    }
}

impl FromStr for PlantAutomaton {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PlantAutomaton, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("initial state: ")?;
        let state = parser
            .scan_to(Token::Whitespace)?
            .chars()
            .zip(0isize..)
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

fn solve(input: Input) -> isize {
    input.parse::<PlantAutomaton>().gen20_sum()
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
            "initial state: #..#.#..##......###...###\n",
            "\n",
            "...## => #\n",
            "..#.. => #\n",
            ".#... => #\n",
            ".#.#. => #\n",
            ".#.## => #\n",
            ".##.. => #\n",
            ".#### => #\n",
            "#.#.# => #\n",
            "#.### => #\n",
            "##.#. => #\n",
            "##.## => #\n",
            "###.. => #\n",
            "###.# => #\n",
            "####. => #\n",
        ));
        assert_eq!(solve(input), 325);
    }
}
