use adventutil::Input;
use adventutil::ranges::parse_range;
use itertools::Itertools;

fn solve(input: Input) -> usize {
    let range = parse_range::<u32>(input.read().trim()).expect("Parse error");
    range.filter(|&n| valid(n)).count()
}

fn valid(n: u32) -> bool {
    let digits = n.to_string().chars().collect::<Vec<_>>();
    if digits.len() != 6 {
        return false;
    }
    let mut has_double = false;
    for (d1, d2) in digits.into_iter().tuple_windows() {
        if d1 == d2 {
            has_double = true;
        }
        if d1 > d2 {
            return false;
        }
    }
    has_double
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(122345, true)]
    #[case(111123, true)]
    #[case(111111, true)]
    #[case(223450, false)]
    #[case(123789, false)]
    fn examples(#[case] n: u32, #[case] v: bool) {
        assert_eq!(valid(n), v);
    }
}
