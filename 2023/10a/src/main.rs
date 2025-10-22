use adventutil::grid::{Direction, Grid};
use adventutil::Input;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Ground,
    Start,
    // Pipes are named based on their exits, i.e., the directions of motion
    // when leaving the tile
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Tile {
    fn follow(self, entrance: Direction) -> Option<Direction> {
        match (self, entrance) {
            (Tile::NorthSouth, Direction::North) => Some(Direction::North),
            (Tile::NorthSouth, Direction::South) => Some(Direction::South),
            (Tile::EastWest, Direction::East) => Some(Direction::East),
            (Tile::EastWest, Direction::West) => Some(Direction::West),
            (Tile::NorthEast, Direction::South) => Some(Direction::East),
            (Tile::NorthEast, Direction::West) => Some(Direction::North),
            (Tile::NorthWest, Direction::South) => Some(Direction::West),
            (Tile::NorthWest, Direction::East) => Some(Direction::North),
            (Tile::SouthWest, Direction::North) => Some(Direction::West),
            (Tile::SouthWest, Direction::East) => Some(Direction::South),
            (Tile::SouthEast, Direction::North) => Some(Direction::East),
            (Tile::SouthEast, Direction::West) => Some(Direction::South),
            _ => None,
        }
    }
}

impl std::str::FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Tile, ParseTileError> {
        match s {
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            "|" => Ok(Tile::NorthSouth),
            "-" => Ok(Tile::EastWest),
            "L" => Ok(Tile::NorthEast),
            "J" => Ok(Tile::NorthWest),
            "7" => Ok(Tile::SouthWest),
            "F" => Ok(Tile::SouthEast),
            _ => Err(ParseTileError(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid tile: {0:?}")]
struct ParseTileError(String);

fn solve(input: Input) -> usize {
    let map = input.parse::<Grid<Tile>>();
    let start = map.iter_cells().find(|&cell| *cell == Tile::Start).unwrap();
    let (mut cell, mut d) = Direction::cardinals()
        .find_map(|d| {
            let n = start.neighbor(d)?;
            let d2 = n.follow(d)?;
            Some((n, d2))
        })
        .unwrap();
    let mut steps = 1;
    loop {
        steps += 1;
        cell = cell.neighbor(d).unwrap();
        if *cell == Tile::Start {
            return steps / 2;
        }
        d = cell.follow(d).unwrap();
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF\n");
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn example2() {
        let input = Input::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...\n");
        assert_eq!(solve(input), 8);
    }
}
