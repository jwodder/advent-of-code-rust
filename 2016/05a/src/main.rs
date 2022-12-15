use adventutil::Input;
use md5::{Digest, Md5};

fn get_pass_char(key: &str, nonce: u32) -> Option<char> {
    let s = format!("{key}{nonce}");
    let digest = hex::encode(Md5::digest(s));
    (&digest[0..5] == "00000").then(|| digest[5..6].chars().next().unwrap())
}

fn solve(input: Input) -> String {
    let key = input.read();
    let key = key.trim();
    (0..)
        .filter_map(|i| get_pass_char(key, i))
        .take(8)
        .collect()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_example1() {
        let input = Input::from("abc");
        assert_eq!(solve(input), "18f47a30");
    }
}
