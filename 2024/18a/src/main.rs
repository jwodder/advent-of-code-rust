use adventutil::grid::{Coords, Direction, GridBounds};
use adventutil::{Input, dijkstra_length};

fn solve(input: Input, bytes: usize, size: usize) -> u32 {
    let bounds = GridBounds::new(size + 1, size + 1);
    let mut corrupted = std::collections::HashSet::with_capacity(bytes);
    for ln in input.lines().take(bytes) {
        let Some((xs, ys)) = ln.trim().split_once(',') else {
            panic!("Line does not contain a comma: {ln:?}");
        };
        let x = xs.parse::<usize>().unwrap();
        let y = ys.parse::<usize>().unwrap();
        corrupted.insert(Coords { y, x });
    }
    dijkstra_length(
        Coords { y: 0, x: 0 },
        |&n| n == Coords { y: size, x: size },
        |&n| {
            Direction::cardinals()
                .filter_map(|d| bounds.move_in(n, d))
                .filter(|c| !corrupted.contains(c))
                .map(|c| (c, 1))
                .collect::<Vec<_>>()
        },
    )
    .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env(), 1024, 70));
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
        assert_eq!(solve(input, 12, 6), 22);
    }
}
