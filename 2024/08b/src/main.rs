use adventutil::grid::{Coords, Grid};
use adventutil::{Input, unordered_pairs};
use std::collections::{HashMap, HashSet};

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>();
    let mut freq2locs: HashMap<char, Vec<Coords>> = HashMap::new();
    for (coord, &c) in &grid {
        if c != '.' {
            freq2locs.entry(c).or_default().push(coord);
        }
    }
    let mut antinodes = HashSet::new();
    for antennae in freq2locs.into_values() {
        for (&c1, &c2) in unordered_pairs(&antennae) {
            antinodes.extend(
                std::iter::zip(jumps(c1.x, c2.x), jumps(c1.y, c2.y))
                    .take_while(|&c| grid.bounds().contains(c))
                    .map(|(x, y)| Coords { x, y }),
            );
            antinodes.extend(
                std::iter::zip(jumps(c2.x, c1.x), jumps(c2.y, c1.y))
                    .take_while(|&c| grid.bounds().contains(c))
                    .map(|(x, y)| Coords { x, y }),
            );
        }
    }
    antinodes.len()
}

fn jumps(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let op = Op::for_diff(a, b);
    std::iter::successors(Some(b), move |&x| op.apply(x))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Add(usize),
    Subtract(usize),
}

impl Op {
    fn for_diff(a: usize, b: usize) -> Op {
        let delta = a.abs_diff(b);
        if b >= a {
            Op::Add(delta)
        } else {
            Op::Subtract(delta)
        }
    }

    fn apply(&self, x: usize) -> Option<usize> {
        match self {
            Op::Add(delta) => x.checked_add(*delta),
            Op::Subtract(delta) => x.checked_sub(*delta),
        }
    }
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
            "............\n",
            "........0...\n",
            ".....0......\n",
            ".......0....\n",
            "....0.......\n",
            "......A.....\n",
            "............\n",
            "............\n",
            "........A...\n",
            ".........A..\n",
            "............\n",
            "............\n",
        ));
        assert_eq!(solve(input), 34);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "T.........\n",
            "...T......\n",
            ".T........\n",
            "..........\n",
            "..........\n",
            "..........\n",
            "..........\n",
            "..........\n",
            "..........\n",
            "..........\n",
        ));
        assert_eq!(solve(input), 9);
    }
}
