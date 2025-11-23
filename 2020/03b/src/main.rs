use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> u64 {
    let grid = <Grid<bool>>::from_drawing(&input.read()).unwrap();
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| intercepts(&grid, right, down))
        .product()
}

fn intercepts(grid: &Grid<bool>, right: usize, down: usize) -> u64 {
    let mut trees = 0;
    let mut cell = grid.get_cell((0, 0));
    while let Some(mut pos) = cell {
        if *pos.get() {
            trees += 1;
        }
        for _ in 0..right {
            pos = pos.east_wrap();
        }
        cell = Some(pos);
        for _ in 0..down {
            cell = cell.and_then(|pos| pos.south());
        }
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
        assert_eq!(solve(input), 336);
    }
}
