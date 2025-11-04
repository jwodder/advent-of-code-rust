// Based on
// <https://www.reddit.com/r/adventofcode/comments/5j4lp1/2016_day_19_solutions/dbdf9mn/>
use adventutil::Input;
use std::collections::VecDeque;

fn solve(input: Input) -> u32 {
    let n = input.parse::<u32>();
    // The current elf and everyone to the left of them
    let mut left = VecDeque::from_iter(1..=(n / 2));
    // Everyone to the right of the current elf, from nearest to farthest
    let mut right = VecDeque::from_iter(((n / 2 + 1)..=n).rev());
    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }
        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }
    left.front().or_else(|| right.front()).copied().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("5");
        assert_eq!(solve(input), 2);
    }
}
