use adventutil::grid::{Coords, Grid};
use adventutil::gridgeom::{PointBounds, Vector};
use adventutil::Input;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    // Clean: not in map
    Weakened,
    Infected,
    Flagged,
}

fn solve(input: Input) -> usize {
    let initial = Grid::from_drawing(&input.read()).unwrap();
    let bounds = initial.bounds();
    let point_bounds = PointBounds {
        min_x: 0,
        max_x: i32::try_from(bounds.width).unwrap(),
        min_y: 0,
        max_y: i32::try_from(bounds.height).unwrap(),
    };
    let mut nodes = initial
        .into_true_coords()
        .map(|c| (point_bounds.at_coords(c, true), State::Infected))
        .collect::<HashMap<_, _>>();
    let mut loc = point_bounds.at_coords(
        Coords {
            y: bounds.height / 2,
            x: bounds.width / 2,
        },
        true,
    );
    let mut d = Vector::NORTH;
    let mut infections_gained = 0;
    for _ in 0..10000000 {
        match nodes.get(&loc).copied() {
            None => {
                d = d.turn_left();
                nodes.insert(loc, State::Weakened);
            }
            Some(State::Weakened) => {
                nodes.insert(loc, State::Infected);
                infections_gained += 1;
            }
            Some(State::Infected) => {
                d = d.turn_right();
                nodes.insert(loc, State::Flagged);
            }
            Some(State::Flagged) => {
                d = -d;
                nodes.remove(&loc);
            }
        }
        loc += d;
    }
    infections_gained
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("..#\n#..\n...\n");
        assert_eq!(solve(input), 2511944);
    }
}
