use adventutil::Input;
use adventutil::grid::Grid;
use std::collections::HashMap;

fn solve(input: Input) -> u64 {
    let grid = input.parse::<Grid<char>>();
    let start = grid
        .iter_cells()
        .find_map(|c| (*c == 'S').then(|| c.coords()))
        .unwrap();
    let mut xes = HashMap::from([(start.x, 1u64)]);
    for y in (start.y + 1)..(grid.height()) {
        let mut new_xes = HashMap::new();
        for (beam, qty) in xes {
            if grid[(y, beam)] == '^' {
                *new_xes.entry(beam - 1).or_default() += qty;
                *new_xes.entry(beam + 1).or_default() += qty;
            } else {
                *new_xes.entry(beam).or_default() += qty;
            }
        }
        xes = new_xes;
    }
    xes.into_values().sum()
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
            ".......S.......\n",
            "...............\n",
            ".......^.......\n",
            "...............\n",
            "......^.^......\n",
            "...............\n",
            ".....^.^.^.....\n",
            "...............\n",
            "....^.^...^....\n",
            "...............\n",
            "...^.^...^.^...\n",
            "...............\n",
            "..^...^.....^..\n",
            "...............\n",
            ".^.^.^.^.^...^.\n",
            "...............\n",
        ));
        assert_eq!(solve(input), 40);
    }
}
