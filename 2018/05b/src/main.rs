use adventutil::Input;
use adventutil::dil::DoubleIndexList;

fn solve(s: &str) -> usize {
    let mut list = DoubleIndexList::new(s.as_bytes());
    b"abcdefghijklmnopqrstuvwxyz"
        .iter()
        .map(|&c| {
            if c > b'a' {
                list.reset();
            }
            let mut cursor = list.cursor();
            while let Some(&sc) = cursor.current() {
                if sc.to_ascii_lowercase() == c {
                    cursor.remove_current();
                } else {
                    cursor.move_next();
                }
            }
            react(&mut list)
        })
        .min()
        .unwrap()
}

fn react(chars: &mut DoubleIndexList<'_, u8>) -> usize {
    let mut cursor = chars.cursor();
    while let Some(c1) = cursor.current().copied() {
        let Some(c2) = cursor.peek_next().copied() else {
            break;
        };
        if c1.is_ascii_lowercase() == c2.is_ascii_uppercase() && c1.eq_ignore_ascii_case(&c2) {
            cursor.remove_two_and_back();
        } else {
            cursor.move_next();
        }
    }
    chars.len()
}

fn main() {
    println!("{}", solve(Input::from_env().read().trim()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve("dabAcCaCBAcCcaDA"), 4);
    }
}
