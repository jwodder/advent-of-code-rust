use adventutil::Input;

fn solve(input: Input) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for ln in input.lines() {
        let mut iter = ln.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse::<u32>().unwrap());
        right.push(iter.next().unwrap().parse::<u32>().unwrap());
        assert_eq!(iter.next(), None);
    }
    left.sort_unstable();
    right.sort_unstable();
    std::iter::zip(left, right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n");
        assert_eq!(solve(input), 11);
    }
}
