use adventutil::grid::{Coords, Direction, Grid};
use adventutil::Input;

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
    let mut pos = start.expect("starting position not found");
    let mut facing = Direction::North;
    let mut visited = std::collections::HashSet::from([pos]);
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
    visited.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
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
        assert_eq!(solve(input), 41);
    }
}
