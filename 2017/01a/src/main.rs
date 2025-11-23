use adventutil::Input;

fn solve(input: Input) -> u32 {
    let digits = input
        .read()
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    digits
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| (c == digits[(i + 1) % digits.len()]).then_some(c))
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
    #[case("1122", 3)]
    #[case("1111", 4)]
    #[case("1234", 0)]
    #[case("91212129", 9)]
    fn examples(#[case] s: &'static str, #[case] total: u32) {
        assert_eq!(solve(Input::from(s)), total);
    }
}
