use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Game {
    players: usize,
    last_marble: usize,
}

impl Game {
    fn high_score(&self) -> usize {
        let mut scores = vec![0; self.players];
        let mut circle = vec![0];
        let mut current = 0;
        for (player, marble) in (0..self.players).cycle().zip(1..=self.last_marble) {
            if marble % 23 == 0 {
                scores[player] += marble;
                current = (current + circle.len() * 7 - 7) % circle.len();
                scores[player] += circle.remove(current);
            } else {
                current = (current + 2) % circle.len();
                circle.insert(current, marble);
            }
        }
        scores.into_iter().max().unwrap()
    }
}

impl std::str::FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Game, ParseError> {
        let mut parser = PullParser::new(s.trim());
        let players = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("players; last marble is worth ")?;
        let last_marble = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("points")?;
        parser.eof()?;
        Ok(Game {
            players,
            last_marble,
        })
    }
}

fn solve(input: Input) -> usize {
    input.parse::<Game>().high_score()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("10 players; last marble is worth 1618 points", 8317)]
    #[case("13 players; last marble is worth 7999 points", 146373)]
    #[case("17 players; last marble is worth 1104 points", 2764)]
    #[case("21 players; last marble is worth 6111 points", 54718)]
    #[case("30 players; last marble is worth 5807 points", 37305)]
    fn high_score(#[case] game: Game, #[case] score: usize) {
        assert_eq!(game.high_score(), score);
    }
}
