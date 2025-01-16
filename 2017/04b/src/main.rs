use adventutil::Input;

fn is_valid(phrase: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    for word in phrase.split_ascii_whitespace() {
        let mut chrs = word.chars().collect::<Vec<_>>();
        chrs.sort_unstable();
        if !seen.insert(chrs) {
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
    #[case("abcde fghij", true)]
    #[case("abcde xyz ecdab", false)]
    #[case("a ab abc abd abf abj", true)]
    #[case("iiii oiii ooii oooi oooo", true)]
    #[case("oiii ioii iioi iiio", false)]
    fn test_is_valid(#[case] phrase: &str, #[case] b: bool) {
        assert_eq!(is_valid(phrase), b);
    }
}
