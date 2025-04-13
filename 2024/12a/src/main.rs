use adventutil::grid::{Direction, Grid};
use adventutil::{components, Input};

fn solve(input: Input) -> usize {
    // Make this a reference to simplify the `move`-ing of `grid` into
    // closures:
    let grid = &input.parse::<Grid<char>>();
    let comps = components(grid.iter_coords(), |c| {
        Direction::cardinals().filter_map(move |d| {
            grid.bounds()
                .move_in(c, d)
                .filter(|&c2| grid[c] == grid[c2])
        })
    });
    let mut price = 0;
    for region in &comps {
        let area = region.len();
        let perimeter: usize = region
            .iter()
            .map(|&c| {
                Direction::cardinals()
                    .filter(move |&d| {
                        grid.bounds()
                            .move_in(c, d)
                            .is_none_or(|c2| !region.contains(&c2))
                    })
                    .count()
            })
            .sum();
        price += area * perimeter;
    }
    price
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("AAAA\nBBCD\nBBCC\nEEEC\n");
        assert_eq!(solve(input), 140);
    }

    #[test]
    fn test_example2() {
        let input = Input::from("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO\n");
        assert_eq!(solve(input), 772);
    }

    #[test]
    fn test_example3() {
        let input = Input::from(concat!(
            "RRRRIICCFF\n",
            "RRRRIICCCF\n",
            "VVRRRCCFFF\n",
            "VVRCCCJFFF\n",
            "VVVVCJJCFE\n",
            "VVIVCCJJEE\n",
            "VVIIICJJEE\n",
            "MIIIIIJJEE\n",
            "MIIISIJEEE\n",
            "MMMISSJEEE\n",
        ));
        assert_eq!(solve(input), 1930);
    }
}
