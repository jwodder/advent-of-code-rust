use adventutil::Input;

fn advance(digits: &[i32]) -> Vec<i32> {
    (0..digits.len())
        .map(|i| {
            let total: i32 = [0i32, 1, 0, -1]
                .into_iter()
                .flat_map(|j| std::iter::repeat_n(j, i + 1))
                .cycle()
                .skip(1)
                .zip(digits.iter())
                .map(|(p, &d)| p * d)
                .sum();
            total.abs() % 10
        })
        .collect()
}

fn solve(input: Input) -> String {
    let mut digits = input
        .read()
        .trim()
        .chars()
        .map(|c| i32::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect::<Vec<_>>();
    for _ in 0..100 {
        digits = advance(&digits);
    }
    digits
        .into_iter()
        .take(8)
        .map(|d| char::from_digit(u32::try_from(d).unwrap(), 10).unwrap())
        .collect()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 8, 2, 2, 6, 1, 5, 8])]
    #[case(vec![4, 8, 2, 2, 6, 1, 5, 8], vec![3, 4, 0, 4, 0, 4, 3, 8])]
    #[case(vec![3, 4, 0, 4, 0, 4, 3, 8], vec![0, 3, 4, 1, 5, 5, 1, 8])]
    #[case(vec![0, 3, 4, 1, 5, 5, 1, 8], vec![0, 1, 0, 2, 9, 4, 9, 8])]
    fn example1(#[case] digits_in: Vec<i32>, #[case] digits_out: Vec<i32>) {
        assert_eq!(advance(&digits_in), digits_out);
    }
}
