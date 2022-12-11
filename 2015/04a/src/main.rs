use adventutil::Input;
use md5::{Digest, Md5};

fn valid(key: &str, nonce: u32) -> bool {
    let s = format!("{key}{nonce}");
    let digest = hex::encode(Md5::digest(&s));
    &digest[0..5] == "00000"
}

fn solve(input: Input) -> u32 {
    let input = input.read();
    let key = input.trim();
    (1..).find(move |&n| valid(key, n)).unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_solve(#[case] s: &'static str, #[case] nonce: u32) {
        let input = Input::from(s);
        assert_eq!(solve(input), nonce);
    }
}
