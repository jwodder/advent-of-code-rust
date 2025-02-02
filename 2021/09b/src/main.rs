use adventutil::grid::{Cell, Direction, Grid};
use adventutil::maxn::maxn;
use adventutil::{one2many_closure, Input};

fn solve(input: Input) -> usize {
    maxn(
        3,
        input
            .parse::<Grid<u32>>()
            .iter_cells()
            .filter(is_low_point)
            .map(basin_size),
    )
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
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "2199943210\n",
            "3987894921\n",
            "9856789892\n",
            "8767896789\n",
            "9899965678\n",
        ));
        assert_eq!(solve(input), 1134);
    }
}
