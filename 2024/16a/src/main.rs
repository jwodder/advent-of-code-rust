use adventutil::grid::{Coords, Direction, Grid, GridFromError};
use adventutil::{Input, dijkstra_length};
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Maze {
    map: Grid<bool>,
    start: Coords,
    end: Coords,
}

impl Maze {
    fn lowest_score(&self) -> u32 {
        let bounds = self.map.bounds();
        dijkstra_length(
            (self.start, Direction::East),
            |&(c, _)| c == self.end,
            |&(c, d)| {
                let mut outputs = vec![((c, d.turn_left()), 1000), ((c, d.turn_right()), 1000)];
                if let Some(c2) = bounds.move_in(c, d).filter(|&c2| self.map[c2]) {
                    outputs.push(((c2, d), 1));
                }
                outputs
            },
        )
        .unwrap()
    }
}

impl std::str::FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(s: &str) -> Result<Maze, ParseMazeError> {
        let mut start = None;
        let mut end = None;
        let mut filled_rows = Vec::new();
        let mut current_row = Vec::new();
        for ch in s.trim_end().chars() {
            match ch {
                '#' => current_row.push(false),
                '.' => current_row.push(true),
                'S' => {
                    let y = filled_rows.len();
                    let x = current_row.len();
                    if start.replace(Coords { y, x }).is_some() {
                        return Err(ParseMazeError::DoubleStart);
                    }
                    current_row.push(true);
                }
                'E' => {
                    let y = filled_rows.len();
                    let x = current_row.len();
                    if end.replace(Coords { y, x }).is_some() {
                        return Err(ParseMazeError::DoubleEnd);
                    }
                    current_row.push(true);
                }
                '\r' => (),
                '\n' => {
                    filled_rows.push(current_row);
                    current_row = Vec::new();
                }
                ch => return Err(ParseMazeError::Unexpected(ch)),
            }
        }
        if !current_row.is_empty() {
            filled_rows.push(current_row);
        }
        Ok(Maze {
            map: Grid::try_from(filled_rows)?,
            start: start.ok_or(ParseMazeError::NoStart)?,
            end: end.ok_or(ParseMazeError::NoEnd)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
enum ParseMazeError {
    #[error("no start in maze")]
    NoStart,
    #[error("multiple starts in maze")]
    DoubleStart,
    #[error("no end in maze")]
    NoEnd,
    #[error("multiple ends in maze")]
    DoubleEnd,
    #[error("unexpected token in maze: {0:?}")]
    Unexpected(char),
    #[error(transparent)]
    Grid(#[from] GridFromError),
}

fn solve(input: Input) -> u32 {
    input.parse::<Maze>().lowest_score()
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
            "###############\n",
            "#.......#....E#\n",
            "#.#.###.#.###.#\n",
            "#.....#.#...#.#\n",
            "#.###.#####.#.#\n",
            "#.#.#.......#.#\n",
            "#.#.#####.###.#\n",
            "#...........#.#\n",
            "###.#.#####.#.#\n",
            "#...#.....#.#.#\n",
            "#.#.#.###.#.#.#\n",
            "#.....#...#.#.#\n",
            "#.###.#.#.#.#.#\n",
            "#S..#.....#...#\n",
            "###############\n",
        ));
        assert_eq!(solve(input), 7036);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "#################\n",
            "#...#...#...#..E#\n",
            "#.#.#.#.#.#.#.#.#\n",
            "#.#.#.#...#...#.#\n",
            "#.#.#.#.###.#.#.#\n",
            "#...#.#.#.....#.#\n",
            "#.#.#.#.#.#####.#\n",
            "#.#...#.#.#.....#\n",
            "#.#.#####.#.###.#\n",
            "#.#.#.......#...#\n",
            "#.#.###.#####.###\n",
            "#.#.#...#.....#.#\n",
            "#.#.#.#####.###.#\n",
            "#.#.#.........#.#\n",
            "#.#.#.#########.#\n",
            "#S#.............#\n",
            "#################\n",
        ));
        assert_eq!(solve(input), 11048);
    }
}
