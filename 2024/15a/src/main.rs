use adventutil::grid::{Coords, Direction, Grid, GridFromError};
use adventutil::Input;
use itertools::Itertools;
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
        match self.map[next_loc] {
            Tile::Empty => {
                self.robot_loc = next_loc;
            }
            Tile::Wall => (),
            Tile::Box => {
                let mut box_cell = self.map.get_cell(next_loc).unwrap();
                while *box_cell == Tile::Box {
                    box_cell = box_cell.neighbor_wrap(d);
                }
                if *box_cell == Tile::Empty {
                    let target_coords = box_cell.coords();
                    self.map[target_coords] = Tile::Box;
                    self.map[next_loc] = Tile::Empty;
                    self.robot_loc = next_loc;
                }
                // else: wall; do nothing
            }
        }
    }

    fn gps_total(&self) -> usize {
        self.map
            .iter()
            .map(|(coords, &tile)| {
                if tile == Tile::Box {
                    coords.y * 100 + coords.x
                } else {
                    0
                }
            })
            .sum()
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
                '#' => current_row.push(Tile::Wall),
                'O' => current_row.push(Tile::Box),
                '.' => current_row.push(Tile::Empty),
                '@' => {
                    let y = filled_rows.len();
                    let x = current_row.len();
                    if robot_loc.replace(Coords { y, x }).is_some() {
                        return Err(ParseWarehouseError::DoubleRobot);
                    }
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
    Box,
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
        let input = Input::from(concat!(
            "########\n",
            "#..O.O.#\n",
            "##@.O..#\n",
            "#...O..#\n",
            "#.#.O..#\n",
            "#...O..#\n",
            "#......#\n",
            "########\n",
            "\n",
            "<^^>>>vv<v>>v<<\n",
        ));
        assert_eq!(solve(input), 2028);
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
        assert_eq!(solve(input), 10092);
    }
}
