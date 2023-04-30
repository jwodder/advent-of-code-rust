use adventutil::Input;
use itertools::{Itertools, MinMaxResult};

fn solve(input: Input) -> u32 {
    input
        .lines()
        .map(|s| {
            match s
                .split_ascii_whitespace()
                .map(|s2| s2.parse::<u32>().unwrap())
                .minmax()
            {
                MinMaxResult::MinMax(x, y) => y - x,
                _ => 0,
            }
        })
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("5 1 9 5\n7 5 3\n2 4 6 8\n");
        assert_eq!(solve(input), 18);
    }
}
