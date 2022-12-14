use adventutil::Input;
use std::collections::HashSet;

fn supports_ssl(s: &str) -> bool {
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for (bracketed, ss) in [false, true].into_iter().cycle().zip(s.split(['[', ']'])) {
        let chars = ss.chars().collect::<Vec<_>>();
        for w in chars.windows(3) {
            if w[0] != w[1] && w[0] == w[2] {
                if bracketed {
                    babs.insert((w[1], w[0]));
                } else {
                    abas.insert((w[0], w[1]));
                }
            }
        }
    }
    !(&abas & &babs).is_empty()
}

fn solve(input: Input) -> usize {
    input.lines().filter(|s| supports_ssl(s)).count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aba[bab]xyz", true)]
    #[case("xyx[xyx]xyx", false)]
    #[case("aaa[kek]eke", true)]
    #[case("zazbz[bzb]cdb", true)]
    fn test_supports_ssl(#[case] ip: &str, #[case] tls: bool) {
        assert_eq!(supports_ssl(ip), tls);
    }
}
