use adventutil::grid::{Coords, Direction, Grid};
use adventutil::Input;
use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap, HashSet};

fn solve(input: Input) -> u32 {
    let mut points = HashMap::new();
    let maze = input
        .read()
        .lines()
        .enumerate()
        .map(|(y, ln)| {
            ln.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if let Some(d) = ch.to_digit(10) {
                        points.insert(d, Coords { y, x });
                        true
                    } else {
                        ch == '.'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let maze = Grid::try_from(maze).unwrap();
    let mut distances = HashMap::new();
    let bounds = maze.bounds();
    for (&p1, &c1) in &points {
        let mut unreached = points
            .keys()
            .copied()
            .filter(|&p2| p1 != p2 && !distances.contains_key(&(p1, p2)))
            .collect::<HashSet<_>>();
        let mut visited = HashSet::new();
        let mut dists = HashMap::from([(c1, 0)]);
        while !unreached.is_empty() {
            let (current, dist) = dists
                .iter()
                .filter(|&(k, _)| !visited.contains(k))
                .min_by_key(|(_, &dist)| dist)
                .map(|(&k, &dist)| (k, dist))
                .unwrap();
            if let Some(p2) = points
                .iter()
                .find_map(|(&p, &c)| (c == current).then_some(p))
            {
                distances.insert((p1, p2), dist);
                distances.insert((p2, p1), dist);
                unreached.remove(&p2);
            }
            for p in Direction::cardinals()
                .filter_map(|d| bounds.move_in(current, d))
                .filter(|&c| maze[c])
            {
                if !visited.contains(&p) {
                    let newdist = dist + 1;
                    match dists.entry(p) {
                        Entry::Vacant(e) => {
                            e.insert(newdist);
                        }
                        Entry::Occupied(mut e) if *e.get() > newdist => {
                            e.insert(newdist);
                        }
                        _ => (),
                    }
                }
            }
            visited.insert(current);
        }
    }
    let dests = points.len() - 1;
    points
        .into_keys()
        .filter(|&p| p != 0)
        .permutations(dests)
        .map(|perm| {
            std::iter::once(0)
                .chain(perm)
                .chain(std::iter::once(0))
                .tuple_windows()
                .map(|(p1, p2)| distances[&(p1, p2)])
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
