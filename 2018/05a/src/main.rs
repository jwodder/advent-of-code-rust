use adventutil::Input;

fn solve(s: &str) -> usize {
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i + 1 < chars.len() {
        let c1 = chars[i];
        let c2 = chars[i + 1];
        if c1.is_ascii_lowercase() == c2.is_ascii_uppercase()
            && c1.to_ascii_uppercase() == c2.to_ascii_uppercase()
        {
            chars.remove(i);
            chars.remove(i);
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
    chars.len()
}

fn main() {
    println!("{}", solve(Input::from_env().read().trim()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aA", 0)]
    #[case("abBA", 0)]
    #[case("abAB", 4)]
    #[case("aabAAB", 6)]
    #[case("dabAcCaCBAcCcaDA", 10)]
    fn test_solve(#[case] s: &str, #[case] units: usize) {
        assert_eq!(solve(s), units);
    }
}
