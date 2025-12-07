use adventutil::Input;
use adventutil::ranges::parse_range;
use itertools::Itertools;

fn solve(input: Input) -> usize {
    let range = parse_range::<u32>(input.read().trim()).expect("Parse error");
    range.filter(|&n| valid(n)).count()
}

fn valid(n: u32) -> bool {
    let digits = n.to_string().chars().collect::<Vec<_>>();
    digits.len() == 6
        && digits
            .iter()
            .copied()
            .tuple_windows()
            .all(|(d1, d2)| d1 <= d2)
        && digits
            .into_iter()
            .chunk_by(|&c| c)
            .into_iter()
            .any(|(_, gr)| gr.count() == 2)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(112233, true)]
    #[case(123444, false)]
    #[case(111122, true)]
    fn examples(#[case] n: u32, #[case] v: bool) {
        assert_eq!(valid(n), v);
    }
}
