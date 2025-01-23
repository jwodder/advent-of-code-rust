use adventutil::Input;

fn knot_hash(key: &[u8]) -> [u8; 32] {
    let mut values = (0u8..=255).collect::<Vec<_>>();
    let mut pos = 0;
    for (skip, length) in std::iter::repeat(key.iter().copied().chain([17, 31, 73, 47, 23]))
        .take(64)
        .flatten()
        .enumerate()
    {
        let length = usize::from(length);
        for i in 0..(length / 2) {
            values.swap((pos + i) % 256, (pos + length - 1 - i) % 256);
        }
        pos = (pos + length + skip) % 256;
    }
    let mut hash = [0u8; 32];
    for (block, cell) in values
        .chunks(16)
        .map(|chunk| chunk.iter().copied().fold(0, |a, b| a ^ b))
        .zip(hash.iter_mut())
    {
        *cell = block;
    }
    hash
}

fn solve(input: Input) -> u32 {
    let keybase = input.read();
    let keybase = keybase.trim();
    (0..128)
        .flat_map(|i| knot_hash(format!("{keybase}-{i}").as_bytes()))
        .map(u8::count_ones)
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("flqrgnkx");
        assert_eq!(solve(input), 8108);
    }
}
