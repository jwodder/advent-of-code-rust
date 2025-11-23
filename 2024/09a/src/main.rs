use adventutil::Input;
use std::collections::VecDeque;

fn solve(input: Input) -> u64 {
    // List of Option<File ID> values
    let mut blocks = VecDeque::new();
    let s = input.read();
    let mut is_file = true;
    let mut i = 0u64;
    for d in s.trim().chars() {
        let size = usize::try_from(d.to_digit(10).unwrap()).unwrap();
        if is_file {
            blocks.extend(std::iter::repeat_n(Some(i), size));
        } else {
            blocks.extend(std::iter::repeat_n(None, size));
            i += 1;
        }
        is_file = !is_file;
    }
    let mut checksum = 0;
    let mut pos = 0u64;
    'outer: while let Some(blk) = blocks.pop_front() {
        if let Some(fid) = blk {
            checksum += pos * fid;
        } else {
            let fid = loop {
                let Some(blk) = blocks.pop_back() else {
                    break 'outer;
                };
                if let Some(fid) = blk {
                    break fid;
                }
            };
            checksum += pos * fid;
        }
        pos += 1;
    }
    checksum
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("2333133121414131402");
        assert_eq!(solve(input), 1928);
    }
}
