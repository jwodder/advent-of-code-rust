use adventutil::Input;

fn solve(input: Input) -> u32 {
    let digits = input
        .read()
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let len = digits.len();
    digits
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| (c == digits[(i + len / 2) % len]).then_some(c))
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1212", 6)]
    #[case("1221", 0)]
    #[case("123425", 4)]
    #[case("123123", 12)]
    #[case("12131415", 4)]
    fn test_solve(#[case] s: &'static str, #[case] total: u32) {
        assert_eq!(solve(Input::from(s)), total);
    }
}
