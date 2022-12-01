use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input, target: usize) -> usize {
    input
        .parse_lines::<usize>()
        .powerset()
        .filter(|comb| comb.iter().sum::<usize>() == target)
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env(), 150));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("20\n15\n10\n5\n5\n");
        assert_eq!(solve(input, 25), 4);
    }
}
