use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> usize {
    let grid = <Grid<bool>>::from_drawing(&input.read()).unwrap();
    let mut trees = 0;
    let mut cell = grid.get_cell((0, 0));
    while let Some(pos) = cell {
        if *pos.get() {
            trees += 1;
        }
        cell = pos.east_wrap().east_wrap().east_wrap().south();
    }
    trees
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
            "..##.......\n",
            "#...#...#..\n",
            ".#....#..#.\n",
            "..#.#...#.#\n",
            ".#...##..#.\n",
            "..#.##.....\n",
            ".#.#.#....#\n",
            ".#........#\n",
            "#.##...#...\n",
            "#...##....#\n",
            ".#..#...#.#\n",
        ));
        assert_eq!(solve(input), 7);
    }
}
