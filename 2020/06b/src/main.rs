use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> usize {
    input
        .paragraphs()
        .map(|para| {
            para.lines()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .reduce(|accum, newset| &accum & &newset)
                .unwrap()
                .len()
        })
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "abc\n", "\n", "a\n", "b\n", "c\n", "\n", "ab\n", "ac\n", "\n", "a\n", "a\n", "a\n",
            "a\n", "\n", "b\n",
        ));
        assert_eq!(solve(input), 6);
    }
}
