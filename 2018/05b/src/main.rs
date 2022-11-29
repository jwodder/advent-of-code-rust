use adventutil::Input;

fn solve(s: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| {
            react(
                s.chars()
                    .filter(move |&sc| sc.to_ascii_lowercase() != c)
                    .collect(),
            )
        })
        .min()
        .unwrap()
}

fn react(mut chars: Vec<char>) -> usize {
    let mut i = 0;
    while i + 1 < chars.len() {
        let c1 = chars[i];
        let c2 = chars[i + 1];
        if c1.is_ascii_lowercase() == c2.is_ascii_uppercase()
            && c1.to_ascii_uppercase() == c2.to_ascii_uppercase()
        {
            chars.remove(i);
            chars.remove(i);
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
    chars.len()
}

fn main() {
    println!("{}", solve(Input::from_env().read().trim()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solve("dabAcCaCBAcCcaDA"), 4);
    }
}
