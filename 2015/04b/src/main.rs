use adventutil::Input;
use md5::{Digest, Md5};

fn valid(key: &str, nonce: u32) -> bool {
    let s = format!("{key}{nonce}");
    let digest = hex::encode(Md5::digest(&s));
    &digest[0..6] == "000000"
}

fn find_nonce(key: &str) -> u32 {
    (1..).find(move |&n| valid(key, n)).unwrap()
}

fn main() {
    println!("{}", find_nonce(Input::from_env().read().trim()));
}
