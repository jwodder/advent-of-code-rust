use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::iter::repeat_n;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Reindeer {
    speed: usize,
    flytime: usize,
    resttime: usize,
}

impl Reindeer {
    fn positions(self) -> impl Iterator<Item = usize> {
        repeat_n(self.speed, self.flytime)
            .chain(repeat_n(0, self.resttime))
            .cycle()
            .scan(0, |sum, d| {
                *sum += d;
                Some(*sum)
            })
    }
}

impl FromStr for Reindeer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Reindeer, ParseError> {
        let mut parser = PullParser::new(s);
        let _name = parser.parse_to::<String, _>(Token::Whitespace)?;
        parser.skip("can fly ")?;
        let speed = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("km/s for ")?;
        let flytime = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("seconds, but then must rest for ")?;
        let resttime = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("seconds.")?;
        parser.eof()?;
        Ok(Reindeer {
            speed,
            flytime,
            resttime,
        })
    }
}

fn solve(input: Input, time: usize) -> usize {
    let deer = input.parse_lines::<Reindeer>().collect::<Vec<_>>();
    let mut scores = vec![0; deer.len()];
    let mut racing = deer
        .into_iter()
        .map(Reindeer::positions)
        .collect::<Vec<_>>();
    for _ in 0..time {
        let positions = racing
            .iter_mut()
            .map(|i| i.next().unwrap())
            .collect::<Vec<_>>();
        let furthest = *positions.iter().max().unwrap();
        for (i, p) in positions.into_iter().enumerate() {
            if p == furthest {
                scores[i] += 1;
            }
        }
    }
    scores.into_iter().max().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env(), 2503));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const COMET: Reindeer = Reindeer {
        speed: 14,
        flytime: 10,
        resttime: 127,
    };
    const DANCER: Reindeer = Reindeer {
        speed: 16,
        flytime: 11,
        resttime: 162,
    };

    #[rstest]
    #[case(COMET, 1, 14)]
    #[case(DANCER, 1, 16)]
    #[case(COMET, 10, 140)]
    #[case(DANCER, 10, 160)]
    #[case(COMET, 11, 140)]
    #[case(DANCER, 11, 176)]
    #[case(COMET, 12, 140)]
    #[case(DANCER, 12, 176)]
    #[case(COMET, 1000, 1120)]
    #[case(DANCER, 1000, 1056)]
    fn test_positions(#[case] deer: Reindeer, #[case] time: usize, #[case] pos: usize) {
        assert_eq!(deer.positions().nth(time - 1).unwrap(), pos);
    }

    #[test]
    fn test_example1a() {
        let input = Input::from(concat!(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.\n",
        ));
        assert_eq!(solve(input, 140), 139);
    }

    #[test]
    fn test_example1b() {
        let input = Input::from(concat!(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.\n",
        ));
        assert_eq!(solve(input, 1000), 689);
    }
}
