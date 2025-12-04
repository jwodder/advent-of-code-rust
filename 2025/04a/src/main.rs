use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    grid.iter_cells()
        .filter(|c| *c == '@' && c.adjacent().filter(|c2| *c2 == '@').count() < 4)
        .count()
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
        assert_eq!(solve(input), 13);
    }
}
