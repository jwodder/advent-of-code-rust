use adventutil::Input;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Command, ParseCommandError> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        if words.len() != 2 {
            return Err(ParseCommandError::WrongWordCount);
        }
        let distance = words[1].parse::<u32>()?;
        match words[0] {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            s => Err(ParseCommandError::BadCommand(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
enum ParseCommandError {
    #[error("Input did not consist of exactly two words")]
    WrongWordCount,
    #[error("Invalid distance: {0}")]
    BadInteger(#[from] ParseIntError),
    #[error("Invalid command: {0:?}")]
    BadCommand(String),
}

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

fn travel<I: IntoIterator<Item = Command>>(iter: I) -> u32 {
    iter.into_iter()
        .fold(Location::new(), |loc, cmd| loc.domove(cmd))
        .product()
}

fn main() {
    let cmds = Input::from_env().parse_lines::<Command>();
    println!("{}", travel(cmds));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let cmds = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .into_iter()
        .map(|s| s.parse::<Command>().unwrap());
        assert_eq!(travel(cmds), 900);
    }
}
