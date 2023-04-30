use adventutil::Input;

fn solve(input: Input) -> usize {
    input
        .parse_lines::<u32>()
        .collect::<Vec<_>>()
        .windows(3)
        .map(|w| w.iter().sum::<u32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(solve(input), 5);
    }
}
