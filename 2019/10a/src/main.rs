use adventutil::grid::Grid;
use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let grid = <Grid<bool>>::from_drawing(&input.read()).unwrap();
    let asteroids = grid
        .enumerate()
        .filter_map(|(coords, &b)| b.then_some(coords))
        .collect::<HashSet<_>>();
    asteroids
        .iter()
        .map(|&coords| {
            asteroids
                .iter()
                .filter(|&&c2| c2 != coords)
                .map(|&c2| {
                    let ydiff = i32::try_from(c2.y).unwrap() - i32::try_from(coords.y).unwrap();
                    let xdiff = i32::try_from(c2.x).unwrap() - i32::try_from(coords.x).unwrap();
                    simplify(ydiff, xdiff)
                })
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap()
}

fn simplify(y: i32, x: i32) -> (i32, i32) {
    if x == 0 && y == 0 {
        return (0, 0);
    } else if x == 0 {
        return (y.signum(), 0);
    } else if y == 0 {
        return (0, x.signum());
    }
    let mut a = y.abs();
    let mut b = x.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    (y / a, x / a)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(".#..#\n.....\n#####\n....#\n...##\n");
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "......#.#.\n",
            "#..#.#....\n",
            "..#######.\n",
            ".#.#.###..\n",
            ".#..#.....\n",
            "..#....#.#\n",
            "#..#....#.\n",
            ".##.#..###\n",
            "##...#..#.\n",
            ".#....####\n",
        ));
        assert_eq!(solve(input), 33);
    }

    #[test]
    fn test_example3() {
        let input = Input::from(concat!(
            "#.#...#.#.\n",
            ".###....#.\n",
            ".#....#...\n",
            "##.#.#.#.#\n",
            "....#.#.#.\n",
            ".##..###.#\n",
            "..#...##..\n",
            "..##....##\n",
            "......#...\n",
            ".####.###.\n",
        ));
        assert_eq!(solve(input), 35);
    }

    #[test]
    fn test_example4() {
        let input = Input::from(concat!(
            ".#..#..###\n",
            "####.###.#\n",
            "....###.#.\n",
            "..###.##.#\n",
            "##.##.#.#.\n",
            "....###..#\n",
            "..#.#..#.#\n",
            "#..#.#.###\n",
            ".##...##.#\n",
            ".....#.#..\n",
        ));
        assert_eq!(solve(input), 41);
    }

    #[test]
    fn test_example5() {
        let input = Input::from(concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##\n",
        ));
        assert_eq!(solve(input), 210);
    }
}
