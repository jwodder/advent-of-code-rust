use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> u32 {
    input
        .lines()
        .map(|s| {
            let compartment1 = s[0..s.len() / 2].chars().collect::<HashSet<_>>();
            let compartment2 = s[s.len() / 2..].chars().collect::<HashSet<_>>();
            priority(
                compartment1
                    .intersection(&compartment2)
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
        assert_eq!(solve(input), 157);
    }
}
