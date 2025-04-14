use adventutil::grid::{Coords, Grid};
use adventutil::{unordered_pairs, Input};
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
            let x = add_diff(c1.x, c2.x);
            let y = add_diff(c1.y, c2.y);
            if let Some((x, y)) = x.zip(y) {
                let c = Coords { x, y };
                if grid.bounds().contains(c) {
                    antinodes.insert(c);
                }
            }
            let x = add_diff(c2.x, c1.x);
            let y = add_diff(c2.y, c1.y);
            if let Some((x, y)) = x.zip(y) {
                let c = Coords { x, y };
                if grid.bounds().contains(c) {
                    antinodes.insert(c);
                }
            }
        }
    }
    antinodes.len()
}

/// Computes `b + (b - a)`
fn add_diff(a: usize, b: usize) -> Option<usize> {
    let delta = a.abs_diff(b);
    match b.cmp(&a) {
        std::cmp::Ordering::Less => b.checked_sub(delta),
        std::cmp::Ordering::Equal => Some(b),
        std::cmp::Ordering::Greater => b.checked_add(delta),
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
        assert_eq!(solve(input), 14);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "..........\n",
            "..........\n",
            "..........\n",
            "....a.....\n",
            "..........\n",
            ".....a....\n",
            "..........\n",
            "..........\n",
            "..........\n",
            "..........\n",
        ));
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_example3() {
        let input = Input::from(concat!(
            "..........\n",
            "..........\n",
            "..........\n",
            "....a.....\n",
            "........a.\n",
            ".....a....\n",
            "..........\n",
            "..........\n",
            "..........\n",
            "..........\n",
        ));
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn test_example4() {
        let input = Input::from(concat!(
            "..........\n",
            "..........\n",
            "..........\n",
            "....a.....\n",
            "........a.\n",
            ".....a....\n",
            "..........\n",
            "......A...\n",
            "..........\n",
            "..........\n",
        ));
        assert_eq!(solve(input), 4);
    }
}
