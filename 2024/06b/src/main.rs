use adventutil::Input;
use adventutil::grid::{Coords, Direction, Grid};
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let mut start = None;
    let map = input
        .read()
        .lines()
        .enumerate()
        .map(|(y, ln)| {
            ln.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '^' {
                        start = Some(Coords { y, x });
                        false
                    } else {
                        ch == '#'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let map = Grid::try_from(map).unwrap();
    let start = start.expect("starting position not found");
    let mut pos = start;
    let mut facing = Direction::North;
    let mut visited = HashSet::from([pos]);
    loop {
        let Some(pos2) = map.bounds().move_in(pos, facing) else {
            break;
        };
        if map[pos2] {
            facing = facing.turn_right();
        } else {
            pos = pos2;
            visited.insert(pos);
        }
    }
    visited
        .into_iter()
        .filter(|&p| {
            let map = add_obstruction(&map, p);
            is_infinite_loop(&map, start)
        })
        .count()
}

fn add_obstruction(map: &Grid<bool>, p: Coords) -> Grid<bool> {
    let mut map = map.clone();
    map[p] = true;
    map
}

fn is_infinite_loop(map: &Grid<bool>, start: Coords) -> bool {
    let mut pos = start;
    let mut facing = Direction::North;
    let mut visited = HashSet::from([(pos, facing)]);
    loop {
        let Some(pos2) = map.bounds().move_in(pos, facing) else {
            return false;
        };
        if map[pos2] {
            facing = facing.turn_right();
        } else {
            pos = pos2;
            if !visited.insert((pos, facing)) {
                return true;
            }
        }
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
        let input = Input::from(concat!(
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "........#.\n",
            "#.........\n",
            "......#...\n",
        ));
        assert_eq!(solve(input), 6);
    }
}
