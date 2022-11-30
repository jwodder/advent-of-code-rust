use adventutil::Input;
use itertools::Itertools;

const ORD_A: u32 = 'a' as u32;

struct PasswordIter {
    current: Vec<char>,
}

impl PasswordIter {
    fn new(s: &str) -> PasswordIter {
        let mut chars = s.chars().collect::<Vec<_>>();
        if let Some(i) = chars.iter().position(|&c| "iol".contains(c)) {
            chars[i] = next_letter(chars[i]);
            chars[(i + 1)..].fill('a');
        }
        PasswordIter {
            current: s.chars().collect(),
        }
    }
}

impl Iterator for PasswordIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        for c in self.current.iter_mut().rev() {
            loop {
                *c = next_letter(*c);
                if !"iol".contains(*c) {
                    break;
                }
            }
            if *c != 'a' {
                return Some(self.current.iter().collect::<String>());
            }
        }
        None
    }
}

fn next_letter(c: char) -> char {
    char::from_u32(((c as u32) - ORD_A + 1) % 26 + ORD_A).unwrap()
}

fn is_valid(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    chars
        .windows(3)
        .any(|w| next_letter(w[0]) == w[1] && next_letter(w[1]) == w[2] && w[2] > 'b')
        && !chars.iter().any(|c| "iol".contains(*c))
        && chars
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| (w[0] == w[1]).then_some(i))
            .tuple_windows()
            .any(|(i, j)| i + 1 < j)
}

fn next_password(s: &str) -> String {
    PasswordIter::new(s).find(|p| is_valid(p)).unwrap()
}

fn main() {
    println!("{}", next_password(Input::from_env().read().trim()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_is_valid(#[case] s: &str, #[case] valid: bool) {
        assert_eq!(is_valid(s), valid);
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_next_password(#[case] before: &str, #[case] after: &str) {
        assert_eq!(next_password(before), after);
    }
}
