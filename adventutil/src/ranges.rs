use crate::pullparser::{ParseError, PullParser, Token};
use num_traits::{Bounded, Num, PrimInt};
use std::ops::{Bound, RangeBounds, RangeInclusive};

pub trait FromBound: Sized {
    fn from_start_bound(bound: Bound<Self>) -> Option<Self>; // returns an inclusive bound
    fn from_end_bound(bound: Bound<Self>) -> Option<Self>; // returns an inclusive bound
}

impl<T: PrimInt> FromBound for T {
    fn from_start_bound(bound: Bound<Self>) -> Option<Self> {
        match bound {
            Bound::Included(x) => Some(x),
            Bound::Excluded(x) => x.checked_add(&T::one()),
            Bound::Unbounded => Some(Bounded::min_value()),
        }
    }

    fn from_end_bound(bound: Bound<Self>) -> Option<Self> {
        match bound {
            Bound::Included(x) => Some(x),
            Bound::Excluded(x) => x.checked_sub(&T::one()),
            Bound::Unbounded => Some(Bounded::max_value()),
        }
    }
}

pub fn ranges_overlap<T, R1, R2>(range1: R1, range2: R2) -> bool
where
    T: PrimInt,
    R1: RangeBounds<T>,
    R2: RangeBounds<T>,
{
    let Some(min1) = T::from_start_bound(range1.start_bound().cloned()) else {
        return false;
    };
    let Some(min2) = T::from_start_bound(range2.start_bound().cloned()) else {
        return false;
    };
    let minimum = min1.max(min2);
    // The following variables are inclusive maximums:
    let Some(max1) = T::from_end_bound(range1.end_bound().cloned()) else {
        return false;
    };
    let Some(max2) = T::from_end_bound(range2.end_bound().cloned()) else {
        return false;
    };
    let maximum = max1.min(max2);
    minimum <= maximum
}

pub fn parse_range<T>(s: &str) -> Result<RangeInclusive<T>, ParseError>
where
    T: Num<FromStrRadixErr = std::num::ParseIntError>,
{
    let mut parser = PullParser::new(s);
    let start = parser.scan_to('-')?;
    let start = T::from_str_radix(start, 10)?;
    let end = parser.scan_to(Token::Eof)?;
    let end = T::from_str_radix(end, 10)?;
    Ok(start..=end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn ranges_overlap() {
        assert!(super::ranges_overlap(0..5, 0..3));
        assert!(super::ranges_overlap(0..3, 0..5));
        assert!(super::ranges_overlap(3..7, 0..10));
        assert!(!super::ranges_overlap(0..3, 3..5));
        assert!(super::ranges_overlap(0..=3, 3..5));
        assert!(super::ranges_overlap::<usize, _, _>(.., ..));
        assert!(!super::ranges_overlap(0..5, 3..0));
        assert!(!super::ranges_overlap(
            (Bound::Excluded(usize::MAX), Bound::Unbounded),
            0..5
        ));
        assert!(!super::ranges_overlap(
            (Bound::Unbounded, Bound::Excluded(usize::MIN)),
            0..5
        ));
        assert!(super::ranges_overlap(
            (Bound::Excluded(usize::MIN), Bound::Unbounded),
            0..5
        ));
        assert!(super::ranges_overlap(
            (Bound::Unbounded, Bound::Excluded(usize::MAX)),
            0..5
        ));
        assert!(super::ranges_overlap(.., 0..5));
    }
}
