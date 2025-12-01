use adventutil::Input;

fn rotate(start: u8, motion: &str) -> (u8, u32) {
    if let Some(offset) = motion.strip_prefix("L").and_then(|s| s.parse::<u32>().ok()) {
        let ticks = offset / 100;
        let offset = u8::try_from(offset % 100).unwrap();
        if start == 0 {
            ((100 - offset) % 100, ticks)
        } else if offset >= start {
            ((start + 100 - offset) % 100, ticks + 1)
        } else {
            (start - offset, ticks)
        }
    } else if let Some(offset) = motion.strip_prefix("R").and_then(|s| s.parse::<u32>().ok()) {
        let ticks = offset / 100;
        let offset = u8::try_from(offset % 100).unwrap();
        let r = start + offset;
        if r >= 100 {
            (r - 100, ticks + 1)
        } else {
            (r, ticks)
        }
    } else {
        panic!("Invalid motion {motion:?}");
    }
}

fn solve(input: Input) -> u32 {
    let mut pos = 50;
    let mut qty = 0;
    for ln in input.lines() {
        let (pos2, ticks) = rotate(pos, &ln);
        pos = pos2;
        qty += ticks;
    }
    qty
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n");
        assert_eq!(solve(input), 6);
    }
}
