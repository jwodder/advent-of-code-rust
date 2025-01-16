use adventutil::Input;

fn is_valid(phrase: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    for word in phrase.split_ascii_whitespace() {
        if !seen.insert(word) {
            return false;
        }
    }
    true
}

fn solve(input: Input) -> usize {
    input.lines().filter(|s| is_valid(s)).count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aa bb cc dd ee", true)]
    #[case("aa bb cc dd aa", false)]
    #[case("aa bb cc dd aaa", true)]
    fn test_is_valid(#[case] phrase: &str, #[case] b: bool) {
        assert_eq!(is_valid(phrase), b);
    }
}
