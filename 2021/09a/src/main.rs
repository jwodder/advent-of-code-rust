use adventutil::grid::{Cell, Direction, Grid};
use adventutil::Input;

fn solve(grid: Grid<u32>) -> u32 {
    grid.iter_cells()
        .filter(is_low_point)
        .map(|c| c.get() + 1)
        .sum()
}

fn is_low_point(cell: &Cell<'_, u32>) -> bool {
    let height = cell.get();
    for d in Direction::cardinals() {
        if let Some(c) = cell.neighbor(d) {
            if *c.get() < *height {
                return false;
            }
        }
    }
    true
}

fn main() {
    let grid = Input::from_env().parse::<Grid<u32>>();
    println!("{}", solve(grid));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = concat!(
            "2199943210\n"
            "3987894921\n"
            "9856789892\n"
            "8767896789\n"
            "9899965678\n"
        )
        .parse::<Grid<u32>>()
        .unwrap();
        assert_eq!(solve(grid), 15);
    }
}
