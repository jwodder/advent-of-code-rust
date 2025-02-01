use adventutil::numtheory::crt;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::str::FromStr;

struct Problem {
    buses: Vec<(i64, i64)>,
}

impl FromStr for Problem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Problem, ParseError> {
        let mut parser = PullParser::new(s);
        let _ = parser.parse_to::<i64, _>(Token::Whitespace)?;
        let buses = parser
            .into_str()
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| {
                (
                    i64::try_from(i).unwrap(),
                    s.parse::<i64>().expect("Parse error"),
                )
            })
            .collect::<Vec<_>>();
        Ok(Problem { buses })
    }
}

fn solve(input: Input) -> i64 {
    let problem = input.parse::<Problem>();
    crt(problem
        .buses
        .into_iter()
        .map(|(i, bus)| (if i == 0 { 0 } else { bus - i }, bus)))
    .unwrap()
    .0
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("939\n7,13,x,x,59,x,31,19\n", 1068781)]
    #[case("0\n17,x,13,19\n", 3417)]
    #[case("0\n67,7,59,61", 754018)]
    #[case("0\n67,x,7,59,61", 779210)]
    #[case("0\n67,7,x,59,61", 1261476)]
    #[case("0\n1789,37,47,1889", 1202161486)]
    fn test_solve(#[case] s: &'static str, #[case] timestamp: i64) {
        assert_eq!(solve(Input::from(s)), timestamp);
    }
}
