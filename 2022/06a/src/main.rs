use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    let chars = input.read().trim().chars().collect::<Vec<_>>();
    for (i, w) in (4..).zip(chars.windows(4)) {
        if w.iter().copied().collect::<HashSet<_>>().len() == 4 {
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
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_solve(#[case] s: &'static str, #[case] output: usize) {
        assert_eq!(solve(Input::from(s)), output);
    }
}
