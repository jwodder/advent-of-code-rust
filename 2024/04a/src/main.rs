use adventutil::Input;
use adventutil::grid::{Direction, Grid};

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<char>>();
    let mut qty = 0;
    for cell in grid.iter_cells() {
        if *cell == 'X' {
            for d in Direction::adjacent() {
                let Some(c2) = cell.neighbor(d) else {
                    continue;
                };
                if *c2 != 'M' {
                    continue;
                }
                let Some(c3) = c2.neighbor(d) else {
                    continue;
                };
                if *c3 != 'A' {
                    continue;
                }
                let Some(c4) = c3.neighbor(d) else {
                    continue;
                };
                if *c4 != 'S' {
                    continue;
                }
                qty += 1;
            }
        }
    }
    qty
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
        assert_eq!(solve(input), 18);
    }
}
