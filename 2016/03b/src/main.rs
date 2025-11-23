use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input) -> usize {
    input
        .lines()
        .map(|s| {
            let sides = s
                .split_ascii_whitespace()
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(sides.len(), 3);
            (sides[0], sides[1], sides[2])
        })
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| {
            let chunk = chunk.collect::<Vec<_>>();
            [
                (chunk[0].0, chunk[1].0, chunk[2].0),
                (chunk[0].1, chunk[1].1, chunk[2].1),
                (chunk[0].2, chunk[1].2, chunk[2].2),
            ]
        })
        .filter(|&(a, b, c)| valid(a, b, c))
        .count()
}

fn valid(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_example1() {
        assert!(!valid(5, 10, 25));
    }
}
