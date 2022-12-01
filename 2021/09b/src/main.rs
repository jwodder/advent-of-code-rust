use adventutil::grid::{Cell, Direction, Grid};
use adventutil::maxn::maxn;
use adventutil::Input;
use std::collections::{HashSet, VecDeque};

fn solve(grid: Grid<u32>) -> usize {
    maxn(3, grid.iter_cells().filter(is_low_point).map(basin_size))
        .into_iter()
        .product()
}

fn is_low_point(cell: &Cell<'_, u32>) -> bool {
    let height = cell.get();
    for d in Direction::cardinals() {
        if let Some(c) = cell.neighbor(d) {
            if *c.get() <= *height {
                return false;
            }
        }
    }
    true
}

fn basin_size(cell: Cell<'_, u32>) -> usize {
    let mut seen = HashSet::new();
    seen.insert(cell.coords());
    let mut queue = VecDeque::new();
    queue.push_back(cell);
    while let Some(c) = queue.pop_front() {
        for d in Direction::cardinals() {
            if let Some(c2) = c.neighbor(d) {
                if *c2.get() < 9 && seen.insert(c2.coords()) {
                    queue.push_back(c2);
                }
            }
        }
    }
    seen.len()
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
            "2199943210\n",
            "3987894921\n",
            "9856789892\n",
            "8767896789\n",
            "9899965678\n",
        )
        .parse::<Grid<u32>>()
        .unwrap();
        assert_eq!(solve(grid), 1134);
    }
}
