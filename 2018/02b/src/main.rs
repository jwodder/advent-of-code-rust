use adventutil::{Input, unordered_pairs};

fn solve(input: Input) -> String {
    let strs = input.lines().collect::<Vec<_>>();
    for (s1, s2) in unordered_pairs(&strs) {
        if let Some(k) = diff_index(s1, s2) {
            return s1
                .chars()
                .enumerate()
                .filter_map(|(i, c)| (i != k).then_some(c))
                .collect();
        }
    }
    panic!("No matching box IDs found");
}

fn diff_index(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut iter = s1
        .chars()
        .zip(s2.chars())
        .enumerate()
        .filter_map(|(i, (c1, c2))| (c1 != c2).then_some(i));
    let k = iter.next()?;
    iter.next().is_none().then_some(k)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n");
        assert_eq!(solve(input), "fgij");
    }
}
