use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Equation {
    target: u64,
    params: Vec<u64>,
}

impl Equation {
    fn solvable(&self) -> bool {
        let first = self.params[0];
        std::iter::repeat_n([Op::Add, Op::Mul, Op::Concat], self.params.len() - 1)
            .multi_cartesian_product()
            .any(|ops| {
                std::iter::zip(ops, &self.params[1..])
                    .fold(first, |left, (op, &right)| op.eval(left, right))
                    == self.target
            })
    }
}

impl std::str::FromStr for Equation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Equation, ParseError> {
        let mut parser = PullParser::new(s);
        let target = parser.parse_to::<u64, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let params = parser.delimited(Token::Whitespace, |word| {
            word.parse::<u64>().map_err(Into::into)
        })?;
        Ok(Equation { target, params })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn eval(&self, left: u64, right: u64) -> u64 {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
            Op::Concat => format!("{left}{right}").parse::<u64>().unwrap(),
        }
    }
}

fn solve(input: Input) -> u64 {
    input
        .parse_lines::<Equation>()
        .filter(Equation::solvable)
        .map(|eq| eq.target)
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "190: 10 19\n",
            "3267: 81 40 27\n",
            "83: 17 5\n",
            "156: 15 6\n",
            "7290: 6 8 6 15\n",
            "161011: 16 10 13\n",
            "192: 17 8 14\n",
            "21037: 9 7 18 13\n",
            "292: 11 6 16 20\n",
        ));
        assert_eq!(solve(input), 11387);
    }
}
