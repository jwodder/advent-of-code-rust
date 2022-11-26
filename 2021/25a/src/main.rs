use adventutil::grid::{Grid, ParseGridError};
use adventutil::Input;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    East,
    South,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::East => write!(f, ">"),
            Cell::South => write!(f, "v"),
        }
    }
}

impl FromStr for Cell {
    type Err = ParseCellError;

    fn from_str(s: &str) -> Result<Cell, ParseCellError> {
        match s {
            "." => Ok(Cell::Empty),
            ">" => Ok(Cell::East),
            "v" => Ok(Cell::South),
            s => Err(ParseCellError(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
#[error("Invalid cell: {0:?}")]
struct ParseCellError(String);

#[derive(Clone, Debug, Eq, PartialEq)]
struct State(Grid<Cell>);

impl State {
    fn step(self) -> State {
        todo!()
    }

    fn stopping_point(mut self) -> usize {
        let mut i = 0;
        loop {
            let next = self.clone().step();
            if self == next {
                return i;
            }
            i += 1;
            self = next;
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for State {
    type Err = ParseGridError<ParseCellError>;

    fn from_str(s: &str) -> Result<State, Self::Err> {
        Ok(State(s.parse()?))
    }
}

fn main() {
    let state = Input::from_env().parse::<State>();
    println!("{}", state.stopping_point());
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
                "....v..\n",
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
                ".......\n",
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
                ".......\n",
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
                "v......\n",
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
                ">.v.v..v.v\n",
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
                "v>.....vv.\n",
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
                ".v>....v..\n",
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
                "vvv>...v..\n",
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
                "v.v.>...v.\n",
            )
        );
    }

    #[test]
    fn test_example4_stopping_point() {
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
        assert_eq!(state.stopping_point(), 58);
    }
}
