use adventutil::Input;
use itertools::Itertools;
use std::collections::HashSet;

fn solve(input: Input) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(s1, s2, s3)| {
            let rs1 = s1.chars().collect::<HashSet<_>>();
            let rs2 = s2.chars().collect::<HashSet<_>>();
            let rs3 = s3.chars().collect::<HashSet<_>>();
            priority(
                rs1.intersection(&rs2.intersection(&rs3).copied().collect())
                    .next()
                    .copied()
                    .unwrap(),
            )
        })
        .sum()
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else if c.is_ascii_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        panic!("Invalid character: {c:?}")
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw\n",
        ));
        assert_eq!(solve(input), 70);
    }
}
