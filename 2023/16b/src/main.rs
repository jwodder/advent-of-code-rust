use adventutil::grid::{Coords, Direction, Grid};
use adventutil::maxtracker::MaxTracker;
use adventutil::Input;
use std::collections::{HashMap, HashSet};

fn energize(grid: &Grid<char>, entry: Coords, entry_dir: Direction) -> usize {
    let mut beams = vec![(grid.get_cell(entry).unwrap(), entry_dir)];
    let mut visited = HashMap::from([(entry, HashSet::from([entry_dir]))]);
    while !beams.is_empty() {
        let mut beams2 = Vec::new();
        for (cell, dir) in beams {
            let outdirs = match (*cell, dir) {
                ('.', dir) => vec![dir],
                ('/', Direction::North) => vec![Direction::East],
                ('/', Direction::East) => vec![Direction::North],
                ('/', Direction::South) => vec![Direction::West],
                ('/', Direction::West) => vec![Direction::South],
                ('\\', Direction::North) => vec![Direction::West],
                ('\\', Direction::East) => vec![Direction::South],
                ('\\', Direction::South) => vec![Direction::East],
                ('\\', Direction::West) => vec![Direction::North],
                ('-', Direction::North | Direction::South) => {
                    vec![Direction::East, Direction::West]
                }
                ('-', dir) => vec![dir],
                ('|', Direction::East | Direction::West) => {
                    vec![Direction::North, Direction::South]
                }
                ('|', dir) => vec![dir],
                (c, d) => panic!("Unexpected tile-dir combo: {c:?}, {d:?}"),
            };
            for d in outdirs {
                if let Some(c2) = cell.neighbor(d) {
                    if visited.entry(c2.coords()).or_default().insert(d) {
                        beams2.push((c2, d));
                    }
                }
            }
        }
        beams = beams2;
    }
    visited.len()
}

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    let mut tracker = MaxTracker::new();
    let max_x = grid.width() - 1;
    let max_y = grid.height() - 1;
    for x in 0..grid.width() {
        tracker.add(energize(&grid, Coords { x, y: 0 }, Direction::South));
        tracker.add(energize(&grid, Coords { x, y: max_y }, Direction::North));
    }
    for y in 0..grid.height() {
        tracker.add(energize(&grid, Coords { x: 0, y }, Direction::East));
        tracker.add(energize(&grid, Coords { x: max_x, y }, Direction::West));
    }
    tracker.get().unwrap()
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
        assert_eq!(solve(input), 51);
    }
}
