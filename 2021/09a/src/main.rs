use adventutil::Input;
use adventutil::grid::{Cell, Grid};

fn solve(input: Input) -> u32 {
    input
        .parse::<Grid<u32>>()
        .iter_cells()
        .filter(is_low_point)
        .map(|c| c.get() + 1)
        .sum()
}

fn is_low_point(cell: &Cell<'_, u32>) -> bool {
    let height = cell.get();
    for c in cell.cardinal_neighbors() {
        if *c.get() <= *height {
            return false;
        }
    }
    true
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
            "2199943210\n",
            "3987894921\n",
            "9856789892\n",
            "8767896789\n",
            "9899965678\n",
        ));
        assert_eq!(solve(input), 15);
    }
}
