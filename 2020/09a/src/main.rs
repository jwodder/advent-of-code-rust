use adventutil::{unordered_pairs, Input};
use std::collections::VecDeque;

fn solve(input: Input, preamble_size: usize) -> u64 {
    let mut iter = input.parse_lines::<u64>();
    let mut preceding = VecDeque::with_capacity(preamble_size + 1);
    for _ in 0..preamble_size {
        preceding.push_back(iter.next().expect("Input lacks complete preamble"));
    }
    for n in iter {
        if !is_sum(preceding.make_contiguous(), n) {
            return n;
        }
        preceding.push_back(n);
        preceding.pop_front();
    }
    panic!("No solution found");
}

fn is_sum(numbers: &[u64], n: u64) -> bool {
    unordered_pairs(numbers).any(|(&n1, &n2)| n1 != n2 && n1 + n2 == n)
}

fn main() {
    println!("{}", solve(Input::from_env(), 25));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "35\n", "20\n", "15\n", "25\n", "47\n", "40\n", "62\n", "55\n", "65\n", "95\n",
            "102\n", "117\n", "150\n", "182\n", "127\n", "219\n", "299\n", "277\n", "309\n",
            "576\n",
        ));
        assert_eq!(solve(input, 5), 127);
    }
}
