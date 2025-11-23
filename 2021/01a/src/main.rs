use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input) -> usize {
    input
        .parse_lines::<u32>()
        .tuple_windows()
        .filter(|(v1, v2)| v1 < v2)
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(solve(input), 7);
    }
}
