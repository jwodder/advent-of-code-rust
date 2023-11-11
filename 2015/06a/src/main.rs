use adventutil::area::{self, Area};
use adventutil::grid::{Coords, Grid, GridBounds};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::str::FromStr;

enum Instruction {
    TurnOn(Coords, Coords),
    TurnOff(Coords, Coords),
    Toggle(Coords, Coords),
}

impl Instruction {
    fn covered(self) -> area::IntoIter<usize> {
        let (c1, c2) = match self {
            Instruction::TurnOn(c1, c2) => (c1, c2),
            Instruction::TurnOff(c1, c2) => (c1, c2),
            Instruction::Toggle(c1, c2) => (c1, c2),
        };
        Area::from_ranges(c1.y..(c2.y + 1), c1.x..(c2.x + 1)).into_iter()
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        let cmd = match parser.scan_to(Token::Whitespace)? {
            cmd @ "toggle" => cmd,
            "turn" => parser.scan_to(Token::Whitespace)?,
            cmd => return Err(ParseError::InvalidToken(cmd.into())),
        };
        let x1 = parser.parse_to::<usize, _>(',')?;
        let y1 = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("through ")?;
        let x2 = parser.parse_to::<usize, _>(',')?;
        let y2 = parser.parse_to::<usize, _>(Token::Eof)?;
        let c1 = Coords::new(y1, x1);
        let c2 = Coords::new(y2, x2);
        match cmd {
            "toggle" => Ok(Instruction::Toggle(c1, c2)),
            "on" => Ok(Instruction::TurnOn(c1, c2)),
            "off" => Ok(Instruction::TurnOff(c1, c2)),
            cmd => Err(ParseError::InvalidToken(cmd.into())),
        }
    }
}

fn solve(input: Input) -> usize {
    let mut lit = Grid::filled(GridBounds::new(1000, 1000), false);
    for i in input.parse_lines::<Instruction>() {
        match i {
            i @ Instruction::Toggle(_, _) => {
                for c in i.covered() {
                    lit[c] ^= true;
                }
            }
            i @ Instruction::TurnOn(_, _) => {
                for c in i.covered() {
                    lit[c] = true;
                }
            }
            i @ Instruction::TurnOff(_, _) => {
                for c in i.covered() {
                    lit[c] = false;
                }
            }
        }
    }
    lit.into_values().filter(|&b| b).count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
