use adventutil::Input;
use itertools::{Itertools, MinMaxResult};

fn solve(input: Input) -> u32 {
    let crabs = input.parse_csv_line::<u32>();
    let (minpos, maxpos) = match crabs.iter().copied().minmax() {
        MinMaxResult::NoElements => panic!("Empty input"),
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(x, y) => (x, y),
    };
    (minpos..=maxpos)
        .map(|pos| {
            crabs
                .iter()
                .map(|&i| {
                    let n = i.abs_diff(pos);
                    n * (n + 1) / 2
                })
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(solve(input), 168);
    }
}
