use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let hits = self.winning.intersection(&self.have).count();
        hits.checked_sub(1).map_or(0, |hits| 1 << hits)
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Card, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Card")?;
        parser.skip(Token::Whitespace)?;
        let id = parser.parse_to::<u32, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let winning = parser
            .scan_to("|")?
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<HashSet<_>, _>>()?;
        let have = parser
            .scan_to(Token::Eof)?
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<HashSet<_>, _>>()?;
        Ok(Card { id, winning, have })
    }
}

fn solve(input: Input) -> u32 {
    input.parse_lines::<Card>().map(|c| c.points()).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
        ));
        assert_eq!(solve(input), 13);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_card(#[case] c: Card, #[case] points: u32) {
        assert_eq!(c.points(), points);
    }
}
