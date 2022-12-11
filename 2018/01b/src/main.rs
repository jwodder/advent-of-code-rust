use adventutil::Input;
use std::collections::HashSet;

fn solve(input: Input) -> i32 {
    let values = input.parse_lines::<i32>().collect::<Vec<_>>();
    let mut seen = HashSet::new();
    let mut sum = 0;
    seen.insert(0);
    loop {
        for &x in &values {
            sum += x;
            if !seen.insert(sum) {
                return sum;
            }
        }
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("+1\n-2\n+3\n+1\n", 2)]
    #[case("+1\n-1\n", 0)]
    #[case("+3\n+3\n+4\n-2\n-4\n", 10)]
    #[case("-6\n+3\n+8\n+5\n-6\n", 5)]
    #[case("+7\n+7\n-2\n-7\n-4\n", 14)]
    fn test_solve(#[case] s: &'static str, #[case] result: i32) {
        let input = Input::from(s);
        assert_eq!(solve(input), result);
    }
}
