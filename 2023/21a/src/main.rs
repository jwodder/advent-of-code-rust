use adventutil::Input;
use adventutil::grid::{Direction, Grid};
use std::collections::HashSet;

fn solve(input: Input, steps: usize) -> usize {
    let mut grid = input.parse::<Grid<char>>();
    let start = grid.iter_coords().find(|&c| grid[c] == 'S').unwrap();
    grid[start] = '.';
    let bounds = grid.bounds();
    let mut locs = HashSet::from([start]);
    for _ in 0..steps {
        locs = locs
            .into_iter()
            .flat_map(|c| Direction::cardinals().filter_map(move |d| bounds.move_in(c, d)))
            .filter(|&c| grid[c] == '.')
            .collect();
    }
    locs.len()
}

fn main() {
    println!("{}", solve(Input::from_env(), 64));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "...........\n",
            ".....###.#.\n",
            ".###.##..#.\n",
            "..#.#...#..\n",
            "....#.#....\n",
            ".##..S####.\n",
            ".##..#...#.\n",
            ".......##..\n",
            ".##.#.####.\n",
            ".##..##.##.\n",
            "...........\n",
        ));
        assert_eq!(solve(input, 6), 16);
    }
}
