use adventutil::Input;
use itertools::Itertools;

fn possible(towels: &[String], pattern: &str) -> bool {
    let mut doable = vec![false; pattern.len() + 1];
    doable[0] = true;
    for i in 0..pattern.len() {
        if doable[i] {
            for tow in towels {
                if pattern[i..].starts_with(tow) {
                    doable[i + tow.len()] = true;
                }
            }
        }
    }
    doable[pattern.len()]
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
