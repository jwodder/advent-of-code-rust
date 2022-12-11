use adventutil::Input;

fn solve(input: Input) -> i32 {
    input.parse_lines::<i32>().sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("+1\n-2\n+3\n+1\n", 3)]
    #[case("+1\n+1\n+1\n", 3)]
    #[case("+1\n+1\n-2\n", 0)]
    #[case("-1\n-2\n-3\n", -6)]
    fn test_solve(#[case] s: &'static str, #[case] result: i32) {
        let input = Input::from(s);
        assert_eq!(solve(input), result);
    }
}
