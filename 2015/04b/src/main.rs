use adventutil::Input;
use md5::{Digest, Md5};

fn valid(key: &str, nonce: u32) -> bool {
    let s = format!("{key}{nonce}");
    let digest = hex::encode(Md5::digest(s));
    &digest[0..6] == "000000"
}

fn solve(input: Input) -> u32 {
    let input = input.read();
    let key = input.trim();
    (1..).find(move |&n| valid(key, n)).unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
