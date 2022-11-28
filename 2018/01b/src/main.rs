use adventutil::Input;
use std::collections::HashSet;

fn first_repeated_sum(values: Vec<i32>) -> i32 {
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
    println!(
        "{}",
        first_repeated_sum(Input::from_env().parse_lines::<i32>().collect())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let values = vec![1, -1];
        assert_eq!(first_repeated_sum(values), 0);
    }

    #[test]
    fn test_example2() {
        let values = vec![3, 3, 4, -2, -4];
        assert_eq!(first_repeated_sum(values), 10);
    }

    #[test]
    fn test_example3() {
        let values = vec![-6, 3, 8, 5, -6];
        assert_eq!(first_repeated_sum(values), 5);
    }

    #[test]
    fn test_example4() {
        let values = vec![7, 7, -2, -7, -4];
        assert_eq!(first_repeated_sum(values), 14);
    }
}
