use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> usize {
    let mut grid = input.parse::<Grid<char>>();
    let mut removed = 0;
    loop {
        let to_remove = grid
            .iter_cells()
            .filter(|c| *c == '@' && c.adjacent().filter(|c2| *c2 == '@').count() < 4)
            .map(|c| c.coords())
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            return removed;
        }
        removed += to_remove.len();
        for c in to_remove {
            grid.set(c, '.');
        }
    }
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
            "..@@.@@@@.\n",
            "@@@.@.@.@@\n",
            "@@@@@.@.@@\n",
            "@.@@@@..@.\n",
            "@@.@@@@.@@\n",
            ".@@@@@@@.@\n",
            ".@.@.@.@@@\n",
            "@.@@@.@@@@\n",
            ".@@@@@@@@.\n",
            "@.@.@@@.@.\n",
        ));
        assert_eq!(solve(input), 43);
    }
}
