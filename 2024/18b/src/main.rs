use adventutil::grid::{Coords, Direction, GridBounds};
use adventutil::{Input, unit_dijkstra_length};
use std::collections::HashSet;

fn solve(input: Input, size: usize) -> String {
    let bounds = GridBounds::new(size + 1, size + 1);
    let mut corrupted = HashSet::new();
    for ln in input.lines() {
        let Some((xs, ys)) = ln.trim().split_once(',') else {
            panic!("Line does not contain a comma: {ln:?}");
        };
        let x = xs.parse::<usize>().unwrap();
        let y = ys.parse::<usize>().unwrap();
        corrupted.insert(Coords { y, x });
        if unit_dijkstra_length(
            Coords { y: 0, x: 0 },
            |&n| n == Coords { y: size, x: size },
            |&n| {
                Direction::cardinals()
                    .filter_map(move |d| bounds.move_in(n, d))
                    .filter(|c| !corrupted.contains(c))
            },
        )
        .is_none()
        {
            return format!("{x},{y}");
        }
    }
    panic!("Exit is always reachable");
}

fn main() {
    println!("{}", solve(Input::from_env(), 70));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "5,4\n", "4,2\n", "4,5\n", "3,0\n", "2,1\n", "6,3\n", "2,4\n", "1,5\n", "0,6\n",
            "3,3\n", "2,6\n", "5,1\n", "1,2\n", "5,5\n", "2,5\n", "6,5\n", "1,4\n", "0,4\n",
            "6,4\n", "1,1\n", "6,1\n", "1,0\n", "0,5\n", "1,6\n", "2,0\n",
        ));
        assert_eq!(solve(input, 6), "6,1");
    }
}
