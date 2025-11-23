use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let chars = input.read().trim().chars().collect::<Vec<_>>();
    for (i, w) in (14..).zip(chars.windows(14)) {
        if w.iter().copied().collect::<HashSet<_>>().len() == 14 {
            return i;
        }
    }
    panic!("Start of packet not found");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn examples(#[case] s: &'static str, #[case] output: usize) {
        assert_eq!(solve(Input::from(s)), output);
    }
}
