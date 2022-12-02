use adventutil::closure::one2many_closure;
use adventutil::grid::{Cell, Direction, Grid};
use adventutil::maxn::maxn;
use adventutil::Input;

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
    let grid = cell.grid();
    let bounds = grid.bounds();
    one2many_closure(cell.coords(), |c| {
        Direction::cardinals()
            .filter_map(move |d| bounds.move_in(c, d))
            .filter(|&c2| *grid.get(c2).unwrap() < 9)
    })
    .len()
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
