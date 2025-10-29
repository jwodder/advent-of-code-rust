use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::grid::Grid;
use itertools::iproduct;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let starting = <Grid<bool>>::from_drawing(&input.read()).unwrap();
    let mut active = starting
        .into_true_coords()
        .map(|coords| {
            (
                i32::try_from(coords.x).unwrap(),
                i32::try_from(coords.y).unwrap(),
                0,
            )
        })
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        let neighbor_qtys = active
            .iter()
            .flat_map(|&(x, y, z)| offsets().map(move |(i, j, k)| (x + i, y + j, z + k)))
            .collect::<Counter<_>>();
        active = neighbor_qtys
            .into_iter()
            .filter_map(|(coords, nqty)| {
                matches!((active.contains(&coords), nqty), (_, 3) | (true, 2)).then_some(coords)
            })
            .collect();
    }
    active.len()
}

fn offsets() -> impl Iterator<Item = (i32, i32, i32)> {
    iproduct!(-1..=1, -1..=1, -1..=1).filter(|&(i, j, k)| (i, j, k) != (0, 0, 0))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(".#.\n..#\n###\n");
        assert_eq!(solve(input), 112);
    }
}
