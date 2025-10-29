use adventutil::Input;
use adventutil::grid::{Direction, Grid};
use std::iter::successors;

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<u32>>();
    grid.iter_cells()
        .filter(|&cell| {
            let &height = cell.get();
            Direction::cardinals().any(|d| {
                successors(Some(cell), |c| c.neighbor(d))
                    .skip(1)
                    .all(|c| *c.get() < height)
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
    fn test_example1() {
        let input = Input::from("30373\n25512\n65332\n33549\n35390\n");
        assert_eq!(solve(input), 21);
    }
}
