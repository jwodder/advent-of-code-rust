use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Reindeer {
    speed: usize,
    flytime: usize,
    resttime: usize,
}

impl Reindeer {
    fn position_at(&self, time: usize) -> usize {
        self.speed
            * (self.flytime * (time / (self.flytime + self.resttime))
                + (time % (self.flytime + self.resttime)).min(self.flytime))
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

fn furthest_at<I: IntoIterator<Item = Reindeer>>(iter: I, time: usize) -> usize {
    iter.into_iter().map(|r| r.position_at(time)).max().unwrap()
}

fn main() {
    println!(
        "{}",
        furthest_at(Input::from_env().parse_lines::<Reindeer>(), 2503)
    );
}

#[cfg(test)]
mod test {
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
    fn test_position_at(#[case] deer: Reindeer, #[case] time: usize, #[case] pos: usize) {
        assert_eq!(deer.position_at(time), pos);
    }

    #[test]
    fn test_example1() {
        assert_eq!(furthest_at([COMET, DANCER], 1000), 1120);
    }
}
