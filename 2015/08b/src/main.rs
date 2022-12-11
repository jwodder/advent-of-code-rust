use adventutil::Input;

fn repr_delta(s: &str) -> usize {
    2 + s.chars().filter(|&c| c == '"' || c == '\\').count()
}

fn solve(input: Input) -> usize {
    input.lines().map(|s| repr_delta(&s)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, 4)]
    #[case(r#""abc""#, 4)]
    #[case(r#""aaa\"aaa""#, 6)]
    #[case(r#""\x27""#, 5)]
    fn test_repr_delta(#[case] s: &str, #[case] delta: usize) {
        assert_eq!(repr_delta(s), delta);
    }
}
