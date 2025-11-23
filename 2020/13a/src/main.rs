use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Problem {
    earliest_time: u32,
    buses: Vec<u32>,
}

impl std::str::FromStr for Problem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Problem, ParseError> {
        let mut parser = PullParser::new(s);
        let earliest_time = parser.parse_to::<u32, _>(Token::Whitespace)?;
        let buses = parser
            .into_str()
            .split(',')
            .filter(|&s| s != "x")
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Problem {
            earliest_time,
            buses,
        })
    }
}

fn solve(input: Input) -> u32 {
    let problem = input.parse::<Problem>();
    let (bus, time) = problem
        .buses
        .into_iter()
        .map(|b| (b, next_eq_mul(b, problem.earliest_time)))
        .min_by_key(|&(_, t)| t)
        .unwrap();
    bus * (time - problem.earliest_time)
}

/// Returns the smallest multiple of `a` greater than or equal to `b`.  Both
/// arguments must be positive integers.
fn next_eq_mul(a: u32, b: u32) -> u32 {
    let c = b % a;
    a + b - (if c == 0 { a } else { c })
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("939\n7,13,x,x,59,x,31,19\n");
        assert_eq!(solve(input), 295);
    }
}
