use adventutil::Input;
use md5::{Digest, Md5};
use std::collections::HashMap;

fn get_pass_char(key: &str, nonce: u32) -> Option<(u32, char)> {
    let s = format!("{key}{nonce}");
    let digest = hex::encode(Md5::digest(s));
    if &digest[0..5] == "00000" {
        let mut iter = digest[5..].chars();
        let c1 = iter.next().unwrap();
        let c2 = iter.next().unwrap();
        if ('0'..'8').contains(&c1) {
            return Some((c1.to_digit(10).unwrap(), c2));
        }
    }
    None
}

fn solve(input: Input) -> String {
    let key = input.read();
    let key = key.trim();
    let mut chars = HashMap::new();
    for (pos, c) in (0..).filter_map(|i| get_pass_char(key, i)) {
        chars.entry(pos).or_insert(c);
        if chars.len() == 8 {
            break;
        }
    }
    (0..8).map(|i| chars[&i]).collect()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example1() {
        let input = Input::from("abc");
        assert_eq!(solve(input), "05ace8e3");
    }
}
