use adventutil::Input;

fn solve(input: Input) -> usize {
    input
        .lines()
        .map(|s| {
            let sides = s
                .split_ascii_whitespace()
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(sides.len(), 3);
            (sides[0], sides[1], sides[2])
        })
        .filter(|&(a, b, c)| valid(a, b, c))
        .count()
}

fn valid(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_example1() {
        assert!(!valid(5, 10, 25));
    }
}
