use adventutil::Input;
use itertools::Itertools;
use std::collections::HashSet;

fn possible(towels: &[String], pattern: &str) -> bool {
    let mut in_progress = HashSet::from([pattern]);
    while !in_progress.is_empty() {
        let mut ip2 = HashSet::new();
        for pat in in_progress {
            if pat.is_empty() {
                return true;
            }
            ip2.extend(towels.iter().filter_map(|tow| pat.strip_prefix(tow)));
        }
        in_progress = ip2;
    }
    false
}

fn solve(input: Input) -> usize {
    let (towels, patterns) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let towels = towels
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect::<Vec<_>>();
    patterns
        .lines()
        .filter(|pat| possible(&towels, pat))
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "r, wr, b, g, bwu, rb, gb, br\n",
            "\n",
            "brwrr\n",
            "bggr\n",
            "gbbr\n",
            "rrbgbr\n",
            "ubwu\n",
            "bwurrg\n",
            "brgr\n",
            "bbrgwb\n",
        ));
        assert_eq!(solve(input), 6);
    }
}
