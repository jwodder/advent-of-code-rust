use adventutil::Input;
use adventutil::grid::{Grid, ParseGridError};
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Octopuses(Grid<u32>);

impl Octopuses {
    fn step(&mut self) -> usize {
        let mut flashed = 0;
        let mut queue = VecDeque::new();
        for coord in self.0.iter_coords() {
            self.0[coord] += 1;
            if self.0[coord] == 10 {
                flashed += 1;
                queue.push_back(coord);
            }
        }
        while let Some(coord) = queue.pop_front() {
            for c in self.0.adjacent_coords(coord) {
                self.0[c] += 1;
                if self.0[c] == 10 {
                    flashed += 1;
                    queue.push_back(c);
                }
            }
        }
        for coord in self.0.iter_coords() {
            if self.0[coord] > 9 {
                self.0[coord] = 0;
            }
        }
        flashed
    }

    fn first_synced_flash(&mut self) -> usize {
        let area = self.0.height() * self.0.width();
        std::iter::repeat_with(|| self.step())
            .position(|f| f == area)
            .expect("No synchronized flash!")
            + 1
    }
}

impl std::str::FromStr for Octopuses {
    type Err = ParseGridError<std::num::ParseIntError>;

    fn from_str(s: &str) -> Result<Octopuses, Self::Err> {
        s.parse::<Grid<u32>>().map(Octopuses)
    }
}

fn solve(input: Input) -> usize {
    input.parse::<Octopuses>().first_synced_flash()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = Input::from(concat!(
            "5483143223\n",
            "2745854711\n",
            "5264556173\n",
            "6141336146\n",
            "6357385478\n",
            "4167524645\n",
            "2176841721\n",
            "6882881134\n",
            "4846848554\n",
            "5283751526\n",
        ));
        assert_eq!(solve(input), 195);
    }
}
