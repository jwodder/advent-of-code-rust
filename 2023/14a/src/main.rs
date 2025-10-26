use adventutil::grid::Grid;
use adventutil::Input;

fn solve(input: Input) -> usize {
    let mut platform = input.parse::<Grid<char>>();
    let mut load = 0;
    for c in platform.iter_coords() {
        if platform[c] == 'O' {
            platform[c] = '.';
            for y in (0..=c.y).rev() {
                if y == 0 || platform[(y - 1, c.x)] != '.' {
                    platform[(y, c.x)] = 'O';
                    load += platform.height() - y;
                    break;
                }
            }
        }
    }
    load
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
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#....\n",
        ));
        assert_eq!(solve(input), 136);
    }
}
