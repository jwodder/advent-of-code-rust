use adventutil::grid::{Coords, Grid, GridBounds};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::{Itertools, Product};
use std::ops::RangeInclusive;
use std::str::FromStr;

enum Instruction {
    TurnOn(Coords, Coords),
    TurnOff(Coords, Coords),
    Toggle(Coords, Coords),
}

impl Instruction {
    fn covered(self) -> Product<RangeInclusive<usize>, RangeInclusive<usize>> {
        let (c1, c2) = match self {
            Instruction::TurnOn(c1, c2) => (c1, c2),
            Instruction::TurnOff(c1, c2) => (c1, c2),
            Instruction::Toggle(c1, c2) => (c1, c2),
        };
        (c1.y..=c2.y).cartesian_product(c1.x..=c2.x)
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

fn follow_instructions<I: IntoIterator<Item = Instruction>>(iter: I) -> usize {
    let mut lit = Grid::<usize>::from_fn::<_, Coords>(GridBounds::new(1000, 1000), |_| 0);
    for i in iter {
        match i {
            i @ Instruction::Toggle(_, _) => {
                for c in i.covered() {
                    lit[c] += 2;
                }
            }
            i @ Instruction::TurnOn(_, _) => {
                for c in i.covered() {
                    lit[c] += 1;
                }
            }
            i @ Instruction::TurnOff(_, _) => {
                for c in i.covered() {
                    lit[c] = lit[c].saturating_sub(1);
                }
            }
        }
    }
    lit.into_values().sum()
}

fn main() {
    println!(
        "{}",
        follow_instructions(Input::from_env().parse_lines::<Instruction>())
    );
}
