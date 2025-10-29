use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>().map(|c| c.to_digit(10));
    grid.iter()
        .filter_map(|(coord, &height)| (height == Some(0)).then_some(coord))
        .map(|coord| {
            let mut locs = Vec::from([coord]);
            for h in 1u32..=9 {
                locs = locs
                    .into_iter()
                    .flat_map(|c| grid.neighbor_coords(c).filter(|&c2| grid[c2] == Some(h)))
                    .collect();
            }
            locs.len()
        })
        .sum()
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
            ".....0.\n",
            "..4321.\n",
            "..5..2.\n",
            "..6543.\n",
            "..7..4.\n",
            "..8765.\n",
            "..9....\n",
        ));
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "..90..9\n",
            "...1.98\n",
            "...2..7\n",
            "6543456\n",
            "765.987\n",
            "876....\n",
            "987....\n",
        ));
        assert_eq!(solve(input), 13);
    }

    #[test]
    fn test_example3() {
        let input = Input::from("012345\n123456\n234567\n345678\n4.6789\n56789.\n");
        assert_eq!(solve(input), 227);
    }

    #[test]
    fn test_example4() {
        let input = Input::from(concat!(
            "89010123\n",
            "78121874\n",
            "87430965\n",
            "96549874\n",
            "45678903\n",
            "32019012\n",
            "01329801\n",
            "10456732\n",
        ));
        assert_eq!(solve(input), 81);
    }
}
