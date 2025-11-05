use adventutil::grid::{Coords, Direction, Grid, GridFromError};
use adventutil::{DistanceMap, Input};
use std::collections::{HashMap, HashSet, hash_map::Entry};
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Maze {
    map: Grid<bool>,
    start: Coords,
    end: Coords,
}

impl Maze {
    fn best_seats(&self) -> usize {
        let mut visited = HashSet::new();
        let start = (self.start, Direction::East);
        let mut distances = DistanceMap::new();
        distances.insert(start, 0);
        let mut paths = ShortestPathTracker::new();
        paths.add(start, 0, HashSet::from([start]));
        while let Some((current, dist)) = distances.pop_nearest() {
            let predecessors = paths.get(&current).unwrap().clone();
            if current.0 == self.end {
                return predecessors
                    .into_iter()
                    .map(|p| p.0)
                    .collect::<HashSet<_>>()
                    .len();
            }
            for (p, d) in self.next_states(current) {
                if !visited.contains(&p) {
                    let mut preds = predecessors.clone();
                    preds.insert(p);
                    paths.add(p, dist + d, preds);
                    distances.insert(p, dist + d);
                }
            }
            visited.insert(current);
        }
        panic!("No route to end");
    }

    fn next_states(
        &self,
        (c, d): (Coords, Direction),
    ) -> impl Iterator<Item = ((Coords, Direction), u32)> {
        let bounds = self.map.bounds();
        [(d, 1), (d.turn_left(), 1001), (d.turn_right(), 1001)]
            .into_iter()
            .filter_map(move |(d2, cost)| {
                bounds
                    .move_in(c, d2)
                    .filter(|&c2| self.map[c2])
                    .map(|c2| ((c2, d2), cost))
            })
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct ShortestPathTracker<T: Eq + std::hash::Hash>(HashMap<T, (u32, HashSet<T>)>);

impl<T: Eq + std::hash::Hash> ShortestPathTracker<T> {
    fn new() -> ShortestPathTracker<T> {
        ShortestPathTracker(HashMap::new())
    }

    fn add(&mut self, node: T, dist: u32, path: HashSet<T>) {
        match self.0.entry(node) {
            Entry::Vacant(e) => {
                e.insert((dist, path));
            }
            Entry::Occupied(mut e) if dist == e.get().0 => {
                e.get_mut().1.extend(path);
            }
            Entry::Occupied(mut e) if dist < e.get().0 => {
                *e.get_mut() = (dist, path);
            }
            _ => (),
        }
    }

    fn get(&self, node: &T) -> Option<&HashSet<T>> {
        self.0.get(node).map(|p| &p.1)
    }
}

fn solve(input: Input) -> usize {
    input.parse::<Maze>().best_seats()
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
        assert_eq!(solve(input), 45);
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
        assert_eq!(solve(input), 64);
    }
}
