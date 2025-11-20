use adventutil::Input;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Outcome {
    winner: Player,
    score: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Player {
    Player1,
    Player2,
}

fn play_game(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> Outcome {
    let mut seen = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        if !seen.insert((deck1.clone(), deck2.clone())) {
            let score = deck1.into_iter().rev().zip(1..).map(|(c, i)| c * i).sum();
            return Outcome {
                winner: Player::Player1,
                score,
            };
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        let round_winner = if card1 <= deck1.len() && card2 <= deck2.len() {
            let subdeck1 = deck1.iter().copied().take(card1).collect();
            let subdeck2 = deck2.iter().copied().take(card2).collect();
            play_game(subdeck1, subdeck2).winner
        } else if card1 > card2 {
            Player::Player1
        } else {
            Player::Player2
        };
        match round_winner {
            Player::Player1 => {
                deck1.push_back(card1);
                deck1.push_back(card2);
            }
            Player::Player2 => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }
    }
    if deck1.is_empty() {
        let score = deck2.into_iter().rev().zip(1..).map(|(c, i)| c * i).sum();
        Outcome {
            winner: Player::Player2,
            score,
        }
    } else {
        let score = deck1.into_iter().rev().zip(1..).map(|(c, i)| c * i).sum();
        Outcome {
            winner: Player::Player1,
            score,
        }
    }
}

fn solve(input: Input) -> usize {
    let (deck1, deck2) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input was not exactly two paragraphs");
    let deck1 = deck1
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("Parse error"))
        .collect::<VecDeque<_>>();
    let deck2 = deck2
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("Parse error"))
        .collect::<VecDeque<_>>();
    play_game(deck1, deck2).score
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
        assert_eq!(solve(input), 291);
    }
}
