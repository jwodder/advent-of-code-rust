use adventutil::Input;
use itertools::Itertools;
use std::collections::HashSet;

fn supports_ssl(s: &str) -> bool {
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for (bracketed, ss) in [false, true].into_iter().cycle().zip(s.split(['[', ']'])) {
        for (c1, c2, c3) in ss.chars().tuple_windows() {
            if c1 != c2 && c1 == c3 {
                if bracketed {
                    babs.insert((c2, c1));
                } else {
                    abas.insert((c1, c2));
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
mod tests {
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
