use adventutil::grid::{Cell, Direction, Grid};
use adventutil::Input;
use std::iter::successors;

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<u32>>();
    grid.iter_cells()
        .map(|cell| {
            Direction::cardinals()
                .map(|d| viewing_distance(cell, d))
                .product::<u32>()
        })
        .max()
        .unwrap()
}

fn viewing_distance(cell: Cell<'_, u32>, direction: Direction) -> u32 {
    let mut i = 0;
    for c in successors(Some(cell), |c| c.neighbor(direction)).skip(1) {
        i += 1;
        if *c >= *cell {
            return i;
        }
    }
    i
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("30373\n25512\n65332\n33549\n35390\n");
        assert_eq!(solve(input), 8);
    }
}
