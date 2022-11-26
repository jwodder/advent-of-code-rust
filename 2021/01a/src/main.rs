use adventutil::Input;

fn main() {
    let values = Input::from_env().parse_lines::<u32>().collect();
    println!("{}", count_increases(values));
}

fn count_increases(values: Vec<u32>) -> usize {
    values.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let values = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(values), 7);
    }
}
