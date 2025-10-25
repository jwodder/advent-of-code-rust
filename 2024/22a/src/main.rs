use adventutil::Input;

type Int = u64;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Buyer(Int);

impl Buyer {
    fn rand(&mut self) -> Int {
        let mut secret = self.0;
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
        self.0 = secret;
        secret
    }
}

impl Iterator for Buyer {
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        Some(self.rand())
    }
}

fn solve(input: Input) -> Int {
    input
        .parse_lines::<Int>()
        .map(|i| Buyer(i).nth(1999).unwrap())
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
        let input = Input::from("1\n10\n100\n2024\n");
        assert_eq!(solve(input), 37327623);
    }
}
