use adventutil::grid::{Cell, Grid, ParseGridError};
use adventutil::Input;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cucumber {
    Empty,
    East,
    South,
}

impl fmt::Display for Cucumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cucumber::Empty => write!(f, "."),
            Cucumber::East => write!(f, ">"),
            Cucumber::South => write!(f, "v"),
        }
    }
}

impl FromStr for Cucumber {
    type Err = ParseCucumberError;

    fn from_str(s: &str) -> Result<Cucumber, ParseCucumberError> {
        match s {
            "." => Ok(Cucumber::Empty),
            ">" => Ok(Cucumber::East),
            "v" => Ok(Cucumber::South),
            s => Err(ParseCucumberError(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
#[error("invalid cell: {0:?}")]
struct ParseCucumberError(String);

#[derive(Clone, Debug, Eq, PartialEq)]
struct State(Grid<Cucumber>);

impl State {
    fn step(&self) -> State {
        State(self.0.map_cells(|cell| match cell.get() {
            Cucumber::Empty if cell.west_wrap() == Cucumber::East => Cucumber::East,
            Cucumber::Empty if moves_south(cell.north_wrap()) => Cucumber::South,
            Cucumber::East if cell.east_wrap() == Cucumber::Empty => {
                match cell.north_wrap().get() {
                    Cucumber::South => Cucumber::South,
                    _ => Cucumber::Empty,
                }
            }
            Cucumber::South if moves_south(cell) => Cucumber::Empty,
            c => *c,
        }))
    }

    fn stopping_point(mut self) -> usize {
        let mut i = 1;
        loop {
            let next = self.step();
            if self == next {
                return i;
            }
            i += 1;
            self = next;
        }
    }
}

fn moves_south(cell: Cell<'_, Cucumber>) -> bool {
    cell == Cucumber::South
        && match cell.south_wrap().get() {
            Cucumber::Empty => cell.south_west_wrap() != Cucumber::East,
            Cucumber::East => cell.south_east_wrap() == Cucumber::Empty,
            Cucumber::South => false,
        }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for State {
    type Err = ParseGridError<ParseCucumberError>;

    fn from_str(s: &str) -> Result<State, Self::Err> {
        Ok(State(s.parse()?))
    }
}

fn solve(input: Input) -> usize {
    input.parse::<State>().stopping_point()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let state = "...>>>>>...".parse::<State>().unwrap();
        let state = state.step();
        assert_eq!(state, "...>>>>.>..".parse().unwrap());
        let state = state.step();
        assert_eq!(state, "...>>>.>.>.".parse().unwrap());
    }

    #[test]
    fn test_example2() {
        let state = concat!(
            "..........\n",
            ".>v....v..\n",
            ".......>..\n",
            "..........\n",
        )
        .parse::<State>()
        .unwrap();
        let state2 = concat!(
            "..........\n",
            ".>........\n",
            "..v....v>.\n",
            "..........\n",
        )
        .parse::<State>()
        .unwrap();
        assert_eq!(state.step(), state2);
    }

    #[test]
    fn test_example3() {
        let state = concat!(
            "...>...\n",
            ".......\n",
            "......>\n",
            "v.....>\n",
            "......>\n",
            ".......\n",
            "..vvv..\n",
        )
        .parse::<State>()
        .unwrap();
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "..vv>..\n",
                ".......\n",
                ">......\n",
                "v.....>\n",
                ">......\n",
                ".......\n",
                "....v..",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "....v>.\n",
                "..vv...\n",
                ".>.....\n",
                "......>\n",
                "v>.....\n",
                ".......\n",
                ".......",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "......>\n",
                "..v.v..\n",
                "..>v...\n",
                ">......\n",
                "..>....\n",
                "v......\n",
                ".......",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                ">......\n",
                "..v....\n",
                "..>.v..\n",
                ".>.v...\n",
                "...>...\n",
                ".......\n",
                "v......",
            )
        );
    }

    #[test]
    fn test_example4_steps() {
        let state = concat!(
            "v...>>.vv>\n",
            ".vv>>.vv..\n",
            ">>.>v>...v\n",
            ">>v>>.>.v.\n",
            "v>v.vv.v..\n",
            ">.>>..v...\n",
            ".vv..>.>v.\n",
            "v.v..>>v.v\n",
            "....v..v.>\n",
        )
        .parse::<State>()
        .unwrap();
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "....>.>v.>\n",
                "v.v>.>v.v.\n",
                ">v>>..>v..\n",
                ">>v>v>.>.v\n",
                ".>v.v...v.\n",
                "v>>.>vvv..\n",
                "..v...>>..\n",
                "vv...>>vv.\n",
                ">.v.v..v.v",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                ">.v.v>>..v\n",
                "v.v.>>vv..\n",
                ">v>.>.>.v.\n",
                ">>v>v.>v>.\n",
                ".>..v....v\n",
                ".>v>>.v.v.\n",
                "v....v>v>.\n",
                ".vv..>>v..\n",
                "v>.....vv.",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "v>v.v>.>v.\n",
                "v...>>.v.v\n",
                ">vv>.>v>..\n",
                ">>v>v.>.v>\n",
                "..>....v..\n",
                ".>.>v>v..v\n",
                "..v..v>vv>\n",
                "v.v..>>v..\n",
                ".v>....v..",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "v>..v.>>..\n",
                "v.v.>.>.v.\n",
                ">vv.>>.v>v\n",
                ">>.>..v>.>\n",
                "..v>v...v.\n",
                "..>>.>vv..\n",
                ">.v.vv>v.v\n",
                ".....>>vv.\n",
                "vvv>...v..",
            )
        );
        let state = state.step();
        assert_eq!(
            state.to_string(),
            concat!(
                "vv>...>v>.\n",
                "v.v.v>.>v.\n",
                ">.v.>.>.>v\n",
                ">v>.>..v>>\n",
                "..v>v.v...\n",
                "..>.>>vvv.\n",
                ".>...v>v..\n",
                "..v.v>>v.v\n",
                "v.v.>...v.",
            )
        );
    }

    #[test]
    fn test_example4_stopping_point() {
        let input = Input::from(concat!(
            "v...>>.vv>\n",
            ".vv>>.vv..\n",
            ">>.>v>...v\n",
            ">>v>>.>.v.\n",
            "v>v.vv.v..\n",
            ">.>>..v...\n",
            ".vv..>.>v.\n",
            "v.v..>>v.v\n",
            "....v..v.>\n",
        ));
        assert_eq!(solve(input), 58);
    }
}
