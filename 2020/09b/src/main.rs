use adventutil::{Input, unordered_pairs};
use itertools::{Itertools, MinMaxResult::*};
use std::cmp::Ordering;

fn solve(input: Input, preamble_size: usize) -> u64 {
    let numbers = input.parse_lines::<u64>().collect::<Vec<_>>();
    let invalid = find_invalid(&numbers, preamble_size);
    for i in 0..numbers.len() {
        let mut total = numbers[i];
        if total > invalid {
            continue;
        }
        for j in (i + 1)..numbers.len() {
            total += numbers[j];
            match total.cmp(&invalid) {
                Ordering::Greater => continue,
                Ordering::Equal => match numbers[i..=j].iter().minmax() {
                    NoElements => unreachable!(),
                    OneElement(x) => return 2 * x,
                    MinMax(x, y) => return x + y,
                },
                Ordering::Less => (),
            }
        }
    }
    panic!("No solution found")
}

fn find_invalid(numbers: &[u64], preamble_size: usize) -> u64 {
    for w in numbers.windows(preamble_size + 1) {
        let (&n, preceding) = w.split_last().unwrap();
        if !is_sum(preceding, n) {
            return n;
        }
    }
    panic!("No invalid number found");
}

fn is_sum(numbers: &[u64], n: u64) -> bool {
    unordered_pairs(numbers).any(|(&n1, &n2)| n1 != n2 && n1 + n2 == n)
}

fn main() {
    println!("{}", solve(Input::from_env(), 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "35\n", "20\n", "15\n", "25\n", "47\n", "40\n", "62\n", "55\n", "65\n", "95\n",
            "102\n", "117\n", "150\n", "182\n", "127\n", "219\n", "299\n", "277\n", "309\n",
            "576\n",
        ));
        assert_eq!(solve(input, 5), 62);
    }
}
