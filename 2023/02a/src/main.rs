use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Color, ParseError> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            other => Err(ParseError::InvalidToken(String::from(other))),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Revelation {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Revelation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Revelation, ParseError> {
        let revs = PullParser::new(s).delimited(", ", |s| {
            let mut parser = PullParser::new(s);
            let qty = parser.parse_to::<u32, _>(Token::Whitespace)?;
            let color = parser.parse_to::<Color, _>(Token::Eof)?;
            parser.eof()?;
            Ok((qty, color))
        })?;
        let mut this = Revelation::default();
        for (qty, color) in revs {
            match color {
                Color::Red => this.red = qty,
                Color::Green => this.green = qty,
                Color::Blue => this.blue = qty,
            }
        }
        Ok(this)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    revealed: Vec<Revelation>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.revealed
            .iter()
            .all(|&rev| rev.red <= 12 && rev.green <= 13 && rev.blue <= 14)
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Game, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Game ")?;
        let id = parser.parse_to::<u32, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let revealed = parser.delimited("; ", str::parse::<Revelation>)?;
        Ok(Game { id, revealed })
    }
}

fn solve(input: Input) -> u32 {
    input
        .parse_lines::<Game>()
        .filter(Game::is_possible)
        .map(|g| g.id)
        .sum()
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        ));
        assert_eq!(solve(input), 8);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_is_possible(#[case] game: Game, #[case] possible: bool) {
        assert_eq!(game.is_possible(), possible);
    }
}
