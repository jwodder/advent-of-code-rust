use adventutil::Input;

fn is_nice(s: &str) -> bool {
    let mut vowels = 0;
    let mut has_double = false;
    let mut prev = None;
    for c in s.chars() {
        if "aeiou".contains(c) {
            vowels += 1;
        }
        if let Some(p) = prev {
            if p == c {
                has_double = true;
            }
            match (p, c) {
                ('a', 'b') => return false,
                ('c', 'd') => return false,
                ('p', 'q') => return false,
                ('x', 'y') => return false,
                _ => (),
            }
        }
        let _ = prev.insert(c);
    }
    vowels >= 3 && has_double
}

fn main() {
    println!(
        "{}",
        Input::from_env().lines().filter(|s| is_nice(s)).count()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_is_nice(#[case] s: &str, #[case] nice: bool) {
        assert_eq!(is_nice(s), nice);
    }
}
