use adventutil::Input;
use itertools::Itertools;

fn possible(towels: &[String], pattern: &str) -> usize {
    let mut qtys = vec![0; pattern.len() + 1];
    qtys[0] = 1;
    for i in 0..pattern.len() {
        for tow in towels {
            if pattern[i..].starts_with(tow) {
                qtys[i + tow.len()] += qtys[i];
            }
        }
    }
    qtys[pattern.len()]
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
    patterns.lines().map(|pat| possible(&towels, pat)).sum()
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
        assert_eq!(solve(input), 16);
    }
}
