use adventutil::grid::{Coords, Direction, Grid, GridFromError};
use adventutil::Input;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Warehouse {
    map: Grid<Tile>,
    robot_loc: Coords,
}

impl Warehouse {
    fn domove(&mut self, d: Direction) {
        let bounds = self.map.bounds();
        // Use `*_wrap` to avoid having to unwrap an Option, which would never
        // be None anyway due to the walls along the warehouse edges.
        let next_loc = bounds.move_in_wrap(self.robot_loc, d);
        let mut next_map = self.map.clone();
        let mut pushing = HashSet::from([self.robot_loc]);
        while !pushing.is_empty() {
            let mut pushing2 = HashSet::new();
            for &c in &pushing {
                let nxt = bounds.move_in_wrap(c, d);
                match self.map[nxt] {
                    Tile::Wall => return,
                    Tile::Empty => {
                        next_map[nxt] = self.map[c];
                    }
                    Tile::BoxLeft => {
                        pushing2.insert(nxt);
                        if matches!(d, Direction::North | Direction::South) {
                            let c2 = bounds.move_in_wrap(c, Direction::East);
                            let nxt2 = bounds.move_in_wrap(c2, d);
                            if !pushing.contains(&c2) {
                                pushing2.insert(nxt2);
                                next_map[nxt2] = Tile::Empty;
                            }
                        }
                        next_map[nxt] = self.map[c];
                    }
                    Tile::BoxRight => {
                        if matches!(d, Direction::North | Direction::South) {
                            let c2 = bounds.move_in_wrap(c, Direction::West);
                            let nxt2 = bounds.move_in_wrap(c2, d);
                            if !pushing.contains(&c2) {
                                pushing2.insert(nxt2);
                                next_map[nxt2] = Tile::Empty;
                            }
                        }
                        pushing2.insert(nxt);
                        next_map[nxt] = self.map[c];
                    }
                }
            }
            pushing = pushing2;
        }
        self.map = next_map;
        self.robot_loc = next_loc;
    }

    fn gps_total(&self) -> usize {
        self.map
            .iter()
            .map(|(coords, &tile)| {
                if tile == Tile::BoxLeft {
                    coords.y * 100 + coords.x
                } else {
                    0
                }
            })
            .sum()
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bounds = self.map.bounds();
        for y in 0..bounds.height {
            for x in 0..bounds.width {
                match self.map[(y, x)] {
                    Tile::Empty => {
                        if self.robot_loc == (Coords { y, x }) {
                            write!(f, "@")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    Tile::Wall => write!(f, "#")?,
                    Tile::BoxLeft => write!(f, "[")?,
                    Tile::BoxRight => write!(f, "]")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Warehouse {
    type Err = ParseWarehouseError;

    fn from_str(s: &str) -> Result<Warehouse, ParseWarehouseError> {
        let mut robot_loc = None;
        let mut filled_rows = Vec::new();
        let mut current_row = Vec::new();
        for ch in s.trim_end().chars() {
            match ch {
                '#' => {
                    current_row.push(Tile::Wall);
                    current_row.push(Tile::Wall);
                }
                'O' => {
                    current_row.push(Tile::BoxLeft);
                    current_row.push(Tile::BoxRight);
                }
                '.' => {
                    current_row.push(Tile::Empty);
                    current_row.push(Tile::Empty);
                }
                '@' => {
                    let y = filled_rows.len();
                    let x = current_row.len();
                    if robot_loc.replace(Coords { y, x }).is_some() {
                        return Err(ParseWarehouseError::DoubleRobot);
                    }
                    current_row.push(Tile::Empty);
                    current_row.push(Tile::Empty);
                }
                '\r' => (),
                '\n' => {
                    filled_rows.push(current_row);
                    current_row = Vec::new();
                }
                ch => return Err(ParseWarehouseError::Unexpected(ch)),
            }
        }
        if !current_row.is_empty() {
            filled_rows.push(current_row);
        }
        Ok(Warehouse {
            map: Grid::try_from(filled_rows)?,
            robot_loc: robot_loc.ok_or(ParseWarehouseError::NoRobot)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
enum ParseWarehouseError {
    #[error("no robot in warehouse map")]
    NoRobot,
    #[error("multiple robots in warehouse map")]
    DoubleRobot,
    #[error("unexpected token in warehouse map: {0:?}")]
    Unexpected(char),
    #[error(transparent)]
    Grid(#[from] GridFromError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
}

fn solve(input: Input) -> usize {
    let (map, motions) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let mut warehouse = map.parse::<Warehouse>().unwrap();
    for ch in motions.chars() {
        match ch {
            '^' => warehouse.domove(Direction::North),
            '>' => warehouse.domove(Direction::East),
            'v' => warehouse.domove(Direction::South),
            '<' => warehouse.domove(Direction::West),
            ch if ch.is_whitespace() => (),
            ch => panic!("Unexpected character in motions: {ch:?}"),
        }
    }
    warehouse.gps_total()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut warehouse = concat!(
            "#######\n",
            "#...#.#\n",
            "#.....#\n",
            "#..OO@#\n",
            "#..O..#\n",
            "#.....#\n",
            "#######\n",
        )
        .parse::<Warehouse>()
        .unwrap();
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##..........##\n",
                "##....[][]@.##\n",
                "##....[]....##\n",
                "##..........##\n",
                "##############\n",
            )
        );

        println!("Moving west ...");
        warehouse.domove(Direction::West);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##..........##\n",
                "##...[][]@..##\n",
                "##....[]....##\n",
                "##..........##\n",
                "##############\n",
            )
        );

        println!("Moving south ...");
        warehouse.domove(Direction::South);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##..........##\n",
                "##...[][]...##\n",
                "##....[].@..##\n",
                "##..........##\n",
                "##############\n",
            )
        );

        println!("Moving south ...");
        warehouse.domove(Direction::South);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##..........##\n",
                "##...[][]...##\n",
                "##....[]....##\n",
                "##.......@..##\n",
                "##############\n",
            )
        );

        println!("Moving west ...");
        warehouse.domove(Direction::West);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##..........##\n",
                "##...[][]...##\n",
                "##....[]....##\n",
                "##......@...##\n",
                "##############\n",
            )
        );

        println!("Moving west ...");
        warehouse.domove(Direction::West);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##..........##\n",
                "##...[][]...##\n",
                "##....[]....##\n",
                "##.....@....##\n",
                "##############\n",
            )
        );

        println!("Moving north ...");
        warehouse.domove(Direction::North);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##...[][]...##\n",
                "##....[]....##\n",
                "##.....@....##\n",
                "##..........##\n",
                "##############\n",
            )
        );

        println!("Moving north ...");
        warehouse.domove(Direction::North);
        assert_eq!(
            warehouse.to_string(),
            concat!(
                "##############\n",
                "##......##..##\n",
                "##...[][]...##\n",
                "##....[]....##\n",
                "##.....@....##\n",
                "##..........##\n",
                "##############\n",
            )
        );
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "##########\n",
            "#..O..O.O#\n",
            "#......O.#\n",
            "#.OO..O.O#\n",
            "#..O@..O.#\n",
            "#O#..O...#\n",
            "#O..O..O.#\n",
            "#.OO.O.OO#\n",
            "#....O...#\n",
            "##########\n",
            "\n",
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n",
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n",
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n",
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n",
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n",
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n",
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n",
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n",
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n",
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n",
        ));
        assert_eq!(solve(input), 9021);
    }
}
