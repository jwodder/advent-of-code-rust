// Idea behind solution: Assume the platform state eventually enters a cycle
// (the graph kind, not the kind we're doing to the platform).  Turns out, it
// does!
use adventutil::grid::{Coords, Direction, Grid, ParseGridError};
use adventutil::{Input, cyclic_nth};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Platform(Grid<char>);

impl Platform {
    fn cycle(&mut self) {
        let height = self.0.height();
        let width = self.0.width();
        self.tilt(
            Direction::North,
            (0..height).flat_map(|y| (0..width).map(move |x| Coords { y, x })),
        );
        self.tilt(
            Direction::West,
            (0..width).flat_map(|x| (0..height).map(move |y| Coords { y, x })),
        );
        self.tilt(
            Direction::South,
            (0..height)
                .rev()
                .flat_map(|y| (0..width).map(move |x| Coords { y, x })),
        );
        self.tilt(
            Direction::East,
            (0..width)
                .rev()
                .flat_map(|x| (0..height).map(move |y| Coords { y, x })),
        );
    }

    fn tilt<I: Iterator<Item = Coords>>(&mut self, dir: Direction, iter: I) {
        let bounds = self.0.bounds();
        for mut c in iter {
            if self.0[c] == 'O' {
                self.0[c] = '.';
                loop {
                    let Some(c2) = bounds.move_in(c, dir) else {
                        break;
                    };
                    if self.0[c2] != '.' {
                        break;
                    }
                    c = c2;
                }
                self.0[c] = 'O';
            }
        }
    }

    fn load(&self) -> usize {
        let height = self.0.height();
        self.0
            .iter_cells()
            .filter(|cell| *cell == 'O')
            .map(|cell| height - cell.coords().y)
            .sum()
    }
}

impl std::str::FromStr for Platform {
    type Err = ParseGridError<<char as std::str::FromStr>::Err>;

    fn from_str(s: &str) -> Result<Platform, Self::Err> {
        s.parse::<Grid<char>>().map(Platform)
    }
}

fn solve(input: Input) -> usize {
    cyclic_nth(
        input.parse::<Platform>(),
        |st| {
            let mut next = st.clone();
            next.cycle();
            next
        },
        1_000_000_000,
    )
    .load()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        let mut state = concat!(
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
        )
        .parse::<Platform>()
        .unwrap();
        state.cycle();
        assert_eq!(
            state.0.to_string(),
            concat!(
                ".....#....\n",
                "....#...O#\n",
                "...OO##...\n",
                ".OO#......\n",
                ".....OOO#.\n",
                ".O#...O#.#\n",
                "....O#....\n",
                "......OOOO\n",
                "#...O###..\n",
                "#..OO#....",
            )
        );
        state.cycle();
        assert_eq!(
            state.0.to_string(),
            concat!(
                ".....#....\n",
                "....#...O#\n",
                ".....##...\n",
                "..O#......\n",
                ".....OOO#.\n",
                ".O#...O#.#\n",
                "....O#...O\n",
                ".......OOO\n",
                "#..OO###..\n",
                "#.OOO#...O",
            )
        );
        state.cycle();
        assert_eq!(
            state.0.to_string(),
            concat!(
                ".....#....\n",
                "....#...O#\n",
                ".....##...\n",
                "..O#......\n",
                ".....OOO#.\n",
                ".O#...O#.#\n",
                "....O#...O\n",
                ".......OOO\n",
                "#...O###.O\n",
                "#.OOO#...O",
            )
        );
    }

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
        assert_eq!(solve(input), 64);
    }
}
