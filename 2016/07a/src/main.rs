use adventutil::Input;

fn supports_tls(s: &str) -> bool {
    let mut has_abba = false;
    let mut has_bracketed_abba = false;
    for (bracketed, ss) in [false, true].into_iter().cycle().zip(s.split(['[', ']'])) {
        let chars = ss.chars().collect::<Vec<_>>();
        for w in chars.windows(4) {
            assert!(w.len() > 3);
            if w[0] != w[1] && w[1] == w[2] && w[0] == w[3] {
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
    fn test_supports_tls(#[case] ip: &str, #[case] tls: bool) {
        assert_eq!(supports_tls(ip), tls);
    }
}
