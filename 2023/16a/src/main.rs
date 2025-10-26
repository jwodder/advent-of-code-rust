use adventutil::grid::{Coords, Direction, Grid};
use adventutil::Input;
use std::collections::{HashMap, HashSet};

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    let mut beams = vec![(grid.get_cell((0, 0)).unwrap(), Direction::East)];
    let mut visited = HashMap::from([(Coords { x: 0, y: 0 }, HashSet::from([Direction::East]))]);
    while !beams.is_empty() {
        let mut beams2 = Vec::new();
        for (cell, mut dir) in beams {
            let mut advance = true;
            match *cell {
                '.' => (),
                '/' => {
                    dir = match dir {
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                        _ => unreachable!(),
                    };
                }
                '\\' => {
                    dir = match dir {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                        _ => unreachable!(),
                    }
                }
                '-' => {
                    if matches!(dir, Direction::North | Direction::South) {
                        for d in [Direction::East, Direction::West] {
                            if let Some(c2) = cell.neighbor(d) {
                                if visited.entry(c2.coords()).or_default().insert(d) {
                                    beams2.push((c2, d));
                                }
                            }
                        }
                        advance = false;
                    }
                }
                '|' => {
                    if matches!(dir, Direction::East | Direction::West) {
                        for d in [Direction::North, Direction::South] {
                            if let Some(c2) = cell.neighbor(d) {
                                if visited.entry(c2.coords()).or_default().insert(d) {
                                    beams2.push((c2, d));
                                }
                            }
                        }
                        advance = false;
                    }
                }
                c => panic!("Unexpected tile {c:?}"),
            }
            if advance {
                if let Some(c2) = cell.neighbor(dir) {
                    if visited.entry(c2.coords()).or_default().insert(dir) {
                        beams2.push((c2, dir));
                    }
                }
            }
        }
        beams = beams2;
    }
    visited.len()
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
            ".|...\\....\n",
            "|.-.\\.....\n",
            ".....|-...\n",
            "........|.\n",
            "..........\n",
            ".........\\\n",
            "..../.\\\\..\n",
            ".-.-/..|..\n",
            ".|....-|.\\\n",
            "..//.|....\n",
        ));
        assert_eq!(solve(input), 46);
    }
}
