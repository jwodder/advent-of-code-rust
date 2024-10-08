use adventutil::Input;
use itertools::Itertools;
use std::fmt::Write;

fn look_and_say(s: &str) -> String {
    s.chars()
        .chunk_by(|&c| c)
        .into_iter()
        .fold(String::new(), |mut buf, (c, run)| {
            write!(&mut buf, "{}{}", run.count(), c).unwrap();
            buf
        })
}

fn solve(input: Input) -> usize {
    let mut s = input.read();
    for _ in 0..40 {
        s = look_and_say(s.trim());
    }
    s.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_look_and_say(#[case] before: &str, #[case] after: &str) {
        assert_eq!(look_and_say(before), after);
    }
}
