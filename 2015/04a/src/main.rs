use adventutil::Input;
use md5::{Digest, Md5};

fn valid(key: &str, nonce: u32) -> bool {
    let s = format!("{key}{nonce}");
    let digest = hex::encode(Md5::digest(&s));
    &digest[0..5] == "00000"
}

fn find_nonce(key: &str) -> u32 {
    (1..).find(move |&n| valid(key, n)).unwrap()
}

fn main() {
    println!("{}", find_nonce(Input::from_env().read().trim()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(find_nonce("abcdef"), 609043);
    }

    #[test]
    fn test_example2() {
        assert_eq!(find_nonce("pqrstuv"), 1048970);
    }
}
