use adventutil::Input;
use std::fmt::Write;

fn solve(input: Input) -> String {
    let lengths = input
        .read()
        .trim()
        .as_bytes()
        .iter()
        .map(|&b| usize::from(b))
        .collect::<Vec<_>>();
    let mut values = (0..256).collect::<Vec<_>>();
    let mut pos = 0;
    for (skip, length) in std::iter::repeat(lengths.into_iter().chain([17, 31, 73, 47, 23]))
        .take(64)
        .flatten()
        .enumerate()
    {
        for i in 0..(length / 2) {
            values.swap((pos + i) % 256, (pos + length - 1 - i) % 256);
        }
        pos = (pos + length + skip) % 256;
    }
    let mut hash = String::with_capacity(32);
    for block in values
        .chunks(16)
        .map(|chunk| chunk.iter().copied().fold(0, |a, b| a ^ b))
    {
        write!(&mut hash, "{block:02x}").unwrap();
    }
    hash
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "a2582a3a0e66e6e86e3812dcb672a272")]
    #[case("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd")]
    #[case("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d")]
    #[case("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e")]
    fn test_example(#[case] s: &'static str, #[case] hash: &str) {
        assert_eq!(solve(Input::from(s)), hash);
    }
}
