use adventutil::Input;
use itertools::Itertools;
use std::collections::HashMap;

fn is_nice(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    let mut pairs = HashMap::new();
    let mut pair_of_pairs = false;
    let mut aba = false;
    let mut prev = None;
    for (i, (c1, c2)) in chars.into_iter().tuple_windows().enumerate() {
        if let Some(c) = prev
            && c2 == c
        {
            aba = true;
        }
        prev.replace(c1);
        let j = *pairs.entry((c1, c2)).or_insert(i);
        if i > j + 1 {
            pair_of_pairs = true;
        }
    }
    pair_of_pairs && aba
}

fn solve(input: Input) -> usize {
    input.lines().filter(|s| is_nice(s)).count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_is_nice(#[case] s: &str, #[case] nice: bool) {
        assert_eq!(is_nice(s), nice);
    }
}
