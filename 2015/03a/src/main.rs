use adventutil::Input;
use std::collections::HashSet;

fn visited(s: &str) -> usize {
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    let mut x = 0;
    let mut y = 0;
    for c in s.chars() {
        match c {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => (),
        }
        seen.insert((x, y));
    }
    seen.len()
}

fn main() {
    println!("{}", visited(&Input::from_env().read()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_visited(#[case] s: &str, #[case] qty: usize) {
        assert_eq!(visited(s), qty);
    }
}
