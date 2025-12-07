use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::grid::Grid;

fn solve(input: Input) -> u64 {
    let grid = input.parse::<Grid<char>>();
    let start = grid
        .iter_cells()
        .find_map(|c| (*c == 'S').then(|| c.coords()))
        .unwrap();
    let mut xes = Counter::new();
    xes.add(start.x);
    for y in (start.y + 1)..(grid.height()) {
        let mut new_xes = Counter::new();
        for (beam, qty) in xes {
            if grid[(y, beam)] == '^' {
                new_xes.add_qty(beam - 1, qty);
                new_xes.add_qty(beam + 1, qty);
            } else {
                new_xes.add_qty(beam, qty);
            }
        }
        xes = new_xes;
    }
    xes.total()
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
