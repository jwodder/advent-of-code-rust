use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn matches(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }
}

impl std::str::FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Card, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Card")?;
        parser.skip(Token::Whitespace)?;
        let id = parser.parse_to::<usize, _>(':')?;
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
    let ids2matches = input
        .parse_lines::<Card>()
        .map(|c| (c.id, c.matches()))
        .collect::<HashMap<_, _>>();
    let mut queue = ids2matches.keys().copied().collect::<VecDeque<_>>();
    let mut cards = 0;
    while let Some(cid) = queue.pop_front() {
        cards += 1;
        let matches = ids2matches[&cid];
        for i in 0..matches {
            queue.push_back(cid + i + 1);
        }
    }
    cards
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
        ));
        assert_eq!(solve(input), 30);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn card_points(#[case] c: Card, #[case] matches: usize) {
        assert_eq!(c.matches(), matches);
    }
}
