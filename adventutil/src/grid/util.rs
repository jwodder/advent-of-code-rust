use std::cmp::Ordering;
use std::ops::Range;

pub(super) fn iurem(x: isize, y: usize) -> usize {
    let y = y
        .try_into()
        .expect("Cannot take remainder with mixed isize and usize: modulus out of range");
    x.rem_euclid(y).try_into().unwrap()
}

pub(super) fn move_in_range(x: usize, range: Range<usize>, delta: Ordering) -> Option<usize> {
    let x = match delta {
        Ordering::Less => x.checked_sub(1)?,
        Ordering::Equal => x,
        Ordering::Greater => x.checked_add(1)?,
    };
    range.contains(&x).then_some(x)
}

pub(super) fn move_in_range_wrap(x: usize, range: Range<usize>, delta: Ordering) -> usize {
    assert!(!range.is_empty(), "Empty range");
    let x = match delta {
        Ordering::Less => x.checked_sub(1).unwrap_or(range.end - 1),
        Ordering::Equal => x,
        Ordering::Greater => x + 1,
    };
    if x < range.start {
        range.end - 1
    } else if x >= range.end {
        range.start
    } else {
        x
    }
}
