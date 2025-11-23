use adventutil::Input;
use itertools::Itertools;

fn supports_tls(s: &str) -> bool {
    let mut has_abba = false;
    let mut has_bracketed_abba = false;
    for (bracketed, ss) in [false, true].into_iter().cycle().zip(s.split(['[', ']'])) {
        for (c1, c2, c3, c4) in ss.chars().tuple_windows() {
            if c1 != c2 && c2 == c3 && c1 == c4 {
                if bracketed {
                    has_bracketed_abba = true;
                } else {
                    has_abba = true;
                }
                break;
            }
        }
    }
    has_abba && !has_bracketed_abba
}

fn solve(input: Input) -> usize {
    input.lines().filter(|s| supports_tls(s)).count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abba[mnop]qrst", true)]
    #[case("abcd[bddb]xyyx", false)]
    #[case("aaaa[qwer]tyui", false)]
    #[case("ioxxoj[asdfgh]zxcvbn", true)]
    fn examples(#[case] ip: &str, #[case] tls: bool) {
        assert_eq!(supports_tls(ip), tls);
    }
}
