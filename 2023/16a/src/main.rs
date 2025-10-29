use adventutil::Input;
use adventutil::grid::{Coords, Direction, Grid};
use std::collections::{HashMap, HashSet};

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    let mut beams = vec![(grid.get_cell((0, 0)).unwrap(), Direction::East)];
    let mut visited = HashMap::from([(Coords { x: 0, y: 0 }, HashSet::from([Direction::East]))]);
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
                if let Some(c2) = cell.neighbor(d)
                    && visited.entry(c2.coords()).or_default().insert(d)
                {
                    beams2.push((c2, d));
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
