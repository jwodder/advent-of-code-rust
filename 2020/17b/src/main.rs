use adventutil::counter::Counter;
use adventutil::grid::Grid;
use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let starting = input.parse::<Grid<char>>();
    let mut active = starting
        .enumerate()
        .filter_map(|(coords, &ch)| {
            (ch == '#').then_some((
                i32::try_from(coords.x).unwrap(),
                i32::try_from(coords.y).unwrap(),
                0,
                0,
            ))
        })
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        let neighbor_qtys = active
            .iter()
            .flat_map(|&(x, y, z, w)| {
                offsets().map(move |(i, j, k, l)| (x + i, y + j, z + k, w + l))
            })
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

fn offsets() -> impl Iterator<Item = (i32, i32, i32, i32)> {
    [-1, 0, 1]
        .into_iter()
        .flat_map(move |i| {
            [-1, 0, 1].into_iter().flat_map(move |j| {
                [-1, 0, 1]
                    .into_iter()
                    .flat_map(move |k| [-1, 0, 1].into_iter().map(move |l| (i, j, k, l)))
            })
        })
        .filter(|&(i, j, k, l)| (i, j, k, l) != (0, 0, 0, 0))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(".#.\n..#\n###\n");
        assert_eq!(solve(input), 848);
    }
}
