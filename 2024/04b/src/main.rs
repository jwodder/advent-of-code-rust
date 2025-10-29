use adventutil::Input;
use adventutil::grid::{Direction, Grid};

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    grid.iter_cells()
        .filter(|cell| {
            *cell == 'A'
                && [Direction::NorthEast, Direction::NorthWest]
                    .into_iter()
                    .all(|d| {
                        let Some(c2) = cell.neighbor(d) else {
                            return false;
                        };
                        let Some(c3) = cell.neighbor(-d) else {
                            return false;
                        };
                        (*c2 == 'M' && *c3 == 'S') || (*c2 == 'S' && *c3 == 'M')
                    })
        })
        .count()
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
            "MMMSXXMASM\n",
            "MSAMXMSMSA\n",
            "AMXSXMAAMM\n",
            "MSAMASMSMX\n",
            "XMASAMXAMM\n",
            "XXAMMXXAMA\n",
            "SMSMSASXSS\n",
            "SAXAMASAAA\n",
            "MAMMMXMMMM\n",
            "MXMXAXMASX\n",
        ));
        assert_eq!(solve(input), 9);
    }
}
