use adventutil::Input;
use adventutil::counter::Counter;
use itertools::Itertools;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    value: [CardRank; 5],
}

impl std::str::FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Hand, ParseHandError> {
        let ranks = s
            .chars()
            .map(CardRank::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let len = ranks.len();
        let Ok(value) = <[CardRank; 5]>::try_from(ranks) else {
            return Err(ParseHandError::Length(len));
        };
        let hand_type = HandType::for_hand(value);
        Ok(Hand { hand_type, value })
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
enum ParseHandError {
    #[error(transparent)]
    Rank(#[from] ParseCardRankError),
    #[error("invalid length for hand: expected 5 cards, got {0}")]
    Length(usize),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandType {
    fn for_hand(value: [CardRank; 5]) -> HandType {
        let mut rank_qtys = Counter::new();
        let mut jokers = 0;
        for r in value {
            if r == CardRank::Joker {
                jokers += 1;
            } else {
                rank_qtys.add(r);
            }
        }
        let mut rank_qtys = rank_qtys.into_values().collect::<Vec<_>>();
        rank_qtys.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
        if rank_qtys.is_empty() {
            return HandType::Five;
        } else {
            rank_qtys[0] += jokers;
        }
        match *rank_qtys {
            [5] => HandType::Five,
            [4, ..] => HandType::Four,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::Three,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::Pair,
            [1, ..] => HandType::HighCard,
            _ => panic!("{value:?} / {rank_qtys:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CardRank {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for CardRank {
    type Error = ParseCardRankError;

    fn try_from(c: char) -> Result<CardRank, ParseCardRankError> {
        match c {
            'J' => Ok(CardRank::Joker),
            '2' => Ok(CardRank::Two),
            '3' => Ok(CardRank::Three),
            '4' => Ok(CardRank::Four),
            '5' => Ok(CardRank::Five),
            '6' => Ok(CardRank::Six),
            '7' => Ok(CardRank::Seven),
            '8' => Ok(CardRank::Eight),
            '9' => Ok(CardRank::Nine),
            'T' => Ok(CardRank::Ten),
            'Q' => Ok(CardRank::Queen),
            'K' => Ok(CardRank::King),
            'A' => Ok(CardRank::Ace),
            c => Err(ParseCardRankError(c)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid card rank: {0:?}")]
struct ParseCardRankError(char);

fn solve(input: Input) -> u32 {
    let mut hands = BTreeMap::new();
    for ln in input.lines() {
        let (word1, word2) = ln.split_whitespace().collect_tuple().unwrap();
        let hand = word1.parse::<Hand>().unwrap();
        let bid = word2.parse::<u32>().unwrap();
        hands.insert(hand, bid);
    }
    hands
        .into_values()
        .zip(1..)
        .map(|(bid, rank)| bid * rank)
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "32T3K 765\n",
            "T55J5 684\n",
            "KK677 28\n",
            "KTJJT 220\n",
            "QQQJA 483\n",
        ));
        assert_eq!(solve(input), 5905);
    }
}
