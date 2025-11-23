use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Command, ParseError> {
        let mut parser = PullParser::new(s);
        let cmd = parser.scan_to(Token::Whitespace)?;
        let distance = parser.parse_to::<u32, _>(Token::Eof)?;
        match cmd {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            _ => Err(ParseError::InvalidToken(cmd.into())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Location {
    hpos: u32,
    depth: u32,
    aim: u32,
}

impl Location {
    fn new() -> Location {
        Location {
            hpos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn domove(self, cmd: Command) -> Location {
        match cmd {
            Command::Forward(d) => Location {
                hpos: self.hpos + d,
                depth: self.depth + self.aim * d,
                ..self
            },
            Command::Down(d) => Location {
                aim: self.aim + d,
                ..self
            },
            Command::Up(d) => Location {
                aim: self.aim.checked_sub(d).expect("We went above the surface!"),
                ..self
            },
        }
    }

    fn product(self) -> u32 {
        self.hpos * self.depth
    }
}

fn solve(input: Input) -> u32 {
    input
        .parse_lines::<Command>()
        .fold(Location::new(), Location::domove)
        .product()
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
            "forward 5\n",
            "down 5\n",
            "forward 8\n",
            "up 3\n",
            "down 8\n",
            "forward 2\n",
        ));
        assert_eq!(solve(input), 900);
    }
}
