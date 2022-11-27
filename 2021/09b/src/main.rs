use adventutil::grid::{Cell, Direction, Grid};
use adventutil::Input;
use std::collections::{HashSet, VecDeque};

struct Max3(VecDeque<usize>);

impl Max3 {
    fn new() -> Max3 {
        Max3(VecDeque::from([0; 3]))
    }

    fn put(&mut self, value: usize) {
        match self.0.binary_search(&value) {
            Ok(0) | Err(0) => (),
            Ok(i) | Err(i) => {
                self.0.insert(i, value);
                self.0.pop_front();
            }
        }
    }

    fn product(self) -> usize {
        self.0.into_iter().product()
    }
}

fn solve(grid: Grid<u32>) -> usize {
    let mut maxes = Max3::new();
    for p in grid.iter_cells().filter(is_low_point) {
        maxes.put(basin_size(p))
    }
    maxes.product()
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
