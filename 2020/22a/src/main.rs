use adventutil::Input;
use itertools::Itertools;
use std::collections::VecDeque;

fn solve(input: Input) -> u32 {
    let (deck1, deck2) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input was not exactly two paragraphs");
    let mut deck1 = deck1
        .lines()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("Parse error"))
        .collect::<VecDeque<_>>();
    let mut deck2 = deck2
        .lines()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("Parse error"))
        .collect::<VecDeque<_>>();
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    let winner = if deck1.is_empty() { deck2 } else { deck1 };
    winner.into_iter().rev().zip(1..).map(|(c, i)| c * i).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "Player 1:\n",
            "9\n",
            "2\n",
            "6\n",
            "3\n",
            "1\n",
            "\n",
            "Player 2:\n",
            "5\n",
            "8\n",
            "4\n",
            "7\n",
            "10\n",
        ));
        assert_eq!(solve(input), 306);
    }
}
