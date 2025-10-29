use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::str::FromStr;

struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn domove(&mut self, m: Movement) {
        let from_stack = &mut self.stacks[m.from_stack - 1];
        let cs = from_stack
            .drain((from_stack.len() - m.qty)..)
            .collect::<Vec<_>>();
        self.stacks[m.to_stack - 1].extend(cs);
    }

    fn top(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|st| st.last().copied())
            .collect()
    }
}

impl FromStr for Crates {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Crates, ParseError> {
        let mut iter = s.lines().rev();
        let columns = iter.next().unwrap().split_ascii_whitespace().count();
        let mut stacks = vec![Vec::new(); columns];
        for ln in iter {
            for (i, c) in ln.chars().skip(1).step_by(4).enumerate() {
                if !c.is_ascii_whitespace() {
                    stacks[i].push(c);
                }
            }
        }
        Ok(Crates { stacks })
    }
}

struct Movement {
    qty: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Movement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Movement, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("move ")?;
        let qty = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("from ")?;
        let from_stack = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("to ")?;
        let to_stack = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Movement {
            qty,
            from_stack,
            to_stack,
        })
    }
}

fn solve(input: Input) -> String {
    let (crates, motions) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let mut crates = crates.parse::<Crates>().expect("Parse error");
    for m in motions
        .lines()
        .map(|s| s.parse::<Movement>().expect("Parse error"))
    {
        crates.domove(m);
    }
    crates.top()
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
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2\n",
        ));
        assert_eq!(solve(input), "MCD");
    }
}
