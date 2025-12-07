use adventutil::Input;
use adventutil::grid::Grid;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    let start = grid
        .iter_cells()
        .find_map(|c| (*c == 'S').then(|| c.coords()))
        .unwrap();
    let mut xes = HashSet::from([start.x]);
    let mut splits = 0;
    for y in (start.y + 1)..(grid.height()) {
        let mut new_xes = HashSet::new();
        for beam in xes {
            if grid[(y, beam)] == '^' {
                splits += 1;
                new_xes.insert(beam - 1);
                new_xes.insert(beam + 1);
            } else {
                new_xes.insert(beam);
            }
        }
        xes = new_xes;
    }
    splits
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
        assert_eq!(solve(input), 21);
    }
}
