use adventutil::Input;

fn knot_hash<I: IntoIterator<Item = usize>>(size: usize, lengths: I) -> usize {
    let mut values = (0..size).collect::<Vec<_>>();
    let mut pos = 0;
    for (skip, length) in lengths.into_iter().enumerate() {
        for i in 0..(length / 2) {
            values.swap((pos + i) % size, (pos + length - 1 - i) % size);
        }
        pos = (pos + length + skip) % size;
    }
    values[0] * values[1]
}

fn solve(input: Input) -> usize {
    knot_hash(256, input.parse_csv_line::<usize>())
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(knot_hash(5, [3, 4, 1, 5]), 12);
    }
}
