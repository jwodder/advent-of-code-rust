use adventutil::Input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Table of which wire × segment mappings are possible
#[derive(Clone, Debug, Eq, PartialEq)]
// `.0[index(wire)][index(segment)]` is true iff the mapping is possible
struct WireSegmentChart(Vec<Vec<bool>>);

impl WireSegmentChart {
    fn new() -> WireSegmentChart {
        WireSegmentChart(vec![vec![true; 7]; 7])
    }

    fn restrict(&mut self, wires: &str, segments: &str) {
        for w in indices(wires) {
            for s in complement(segments) {
                self.0[w][s] = false;
            }
        }
        for w in complement(wires) {
            for s in indices(segments) {
                self.0[w][s] = false;
            }
        }
    }

    fn solution(&self) -> Option<HashMap<char, char>> {
        let mut map = HashMap::new();
        for (wire, segments) in std::iter::zip('a'..='g', &self.0) {
            let seg = std::iter::zip('a'..='g', segments)
                .filter(|&(_, &b)| b)
                .map(|(s, _)| s)
                .exactly_one()
                .ok()?;
            map.insert(wire, seg);
        }
        Some(map)
    }
}

fn indices(s: &str) -> impl Iterator<Item = usize> {
    s.chars()
        .map(|c| usize::try_from(u32::from(c) - 0x61).unwrap())
}

fn complement(s: &str) -> impl Iterator<Item = usize> {
    ('a'..='g')
        .filter(|&c| !s.contains(c))
        .map(|c| usize::try_from(u32::from(c) - 0x61).unwrap())
}

fn solve_line(s: &str) -> u32 {
    let (patterns, output) = s.split_once(" | ").unwrap();
    let patterns = patterns.split_whitespace().collect::<Vec<_>>();
    let output = output.split_whitespace().collect::<Vec<_>>();
    let mut patterns_by_len = HashMap::<usize, Vec<&str>>::new();
    for pat in patterns {
        patterns_by_len.entry(pat.len()).or_default().push(pat);
    }
    let mut map = WireSegmentChart::new();
    let [one_pattern] = *patterns_by_len[&2] else {
        panic!("No unique two-character pattern");
    };
    map.restrict(one_pattern, "cf");
    let [four_pattern] = *patterns_by_len[&4] else {
        panic!("No unique four-character pattern");
    };
    map.restrict(four_pattern, "bcdf");
    let [seven_pattern] = *patterns_by_len[&3] else {
        panic!("No unique three-character pattern");
    };
    map.restrict(seven_pattern, "acf");
    // The wires that are common to all five-length patterns can only be the
    // segments that are common to all five-length displays, which are "adg".
    let common_fiver_wires = patterns_by_len[&5]
        .iter()
        .fold(HashSet::<char>::from_iter('a'..='g'), |mut common, s| {
            common.retain(|&c| s.contains(c));
            common
        })
        .into_iter()
        .collect::<String>();
    map.restrict(&common_fiver_wires, "adg");
    // The wires that are common to all six-length patterns can only be the
    // segments that are common to all six-length displays, which are "abfg".
    let common_sixer_wires = patterns_by_len[&6]
        .iter()
        .fold(HashSet::<char>::from_iter('a'..='g'), |mut common, s| {
            common.retain(|&c| s.contains(c));
            common
        })
        .into_iter()
        .collect::<String>();
    map.restrict(&common_sixer_wires, "abfg");
    let Some(assignments) = map.solution() else {
        panic!("No solution found for {s:?}");
    };
    let mut num = 0;
    for out in output {
        let mut segments = out.chars().map(|ch| assignments[&ch]).collect::<Vec<_>>();
        segments.sort_unstable();
        let d = match &*String::from_iter(segments) {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            s => panic!("Invalid decoded segments {s:?}"),
        };
        num = num * 10 + d;
    }
    num
}

fn solve(input: Input) -> u32 {
    input.lines().map(|ln| solve_line(&ln)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(solve_line(s), 5353);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n",
        ));
        assert_eq!(solve(input), 61229);
    }
}
