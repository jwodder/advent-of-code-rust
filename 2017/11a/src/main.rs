use adventutil::Input;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct HexCoord {
    // On rows with odd `y` values, only cells with odd `x` values exist, and
    // likewise for evens.
    x: i32,
    y: i32,
}

impl HexCoord {
    fn step(self, st: Step) -> HexCoord {
        match st {
            Step::North => HexCoord {
                x: self.x,
                y: self.y + 2,
            },
            Step::NorthEast => HexCoord {
                x: self.x + 1,
                y: self.y + 1,
            },
            Step::SouthEast => HexCoord {
                x: self.x + 1,
                y: self.y - 1,
            },
            Step::South => HexCoord {
                x: self.x,
                y: self.y - 2,
            },
            Step::SouthWest => HexCoord {
                x: self.x - 1,
                y: self.y - 1,
            },
            Step::NorthWest => HexCoord {
                x: self.x - 1,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Step {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl std::str::FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Step, ParseStepError> {
        match s {
            "n" => Ok(Step::North),
            "ne" => Ok(Step::NorthEast),
            "se" => Ok(Step::SouthEast),
            "s" => Ok(Step::South),
            "sw" => Ok(Step::SouthWest),
            "nw" => Ok(Step::NorthWest),
            _ => Err(ParseStepError(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid step: {0:?}")]
struct ParseStepError(String);

fn solve(input: Input) -> i32 {
    let mut loc = HexCoord { x: 0, y: 0 };
    for st in input.parse_csv_line::<Step>() {
        loc = loc.step(st);
    }
    let mut y = loc.y.abs();
    let mut x = loc.x.abs();
    let diag_steps = std::cmp::min(x, y);
    y -= diag_steps;
    x -= diag_steps;
    let vert_steps = y / 2 + x;
    diag_steps + vert_steps
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ne,ne,ne", 3)]
    #[case("ne,ne,sw,sw", 0)]
    #[case("ne,ne,s,s", 2)]
    #[case("se,sw,se,sw,sw", 3)]
    fn examples(#[case] inp: &'static str, #[case] answer: i32) {
        let input = Input::from(inp);
        assert_eq!(solve(input), answer);
    }
}
