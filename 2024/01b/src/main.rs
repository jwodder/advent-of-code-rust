use adventutil::Input;
use adventutil::counter::Counter;

fn solve(input: Input) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for ln in input.lines() {
        let mut iter = ln.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse::<u64>().unwrap());
        right.push(iter.next().unwrap().parse::<u64>().unwrap());
        assert_eq!(iter.next(), None);
    }
    let freqs = Counter::from_iter(right);
    left.into_iter().map(|n| n * freqs[&n]).sum()
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
        assert_eq!(solve(input), 31);
    }
}
