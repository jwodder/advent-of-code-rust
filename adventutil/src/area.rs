use num_traits::{one, zero, NumCast, PrimInt};
use std::iter::FusedIterator;
use std::ops::Range;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Area<T> {
    pub start_x: T, // inclusive
    pub end_x: T,   // exclusive
    pub start_y: T, // inclusive
    pub end_y: T,   // exclusive
}

impl<T: PrimInt> Area<T> {
    pub fn from_ranges(width: Range<T>, height: Range<T>) -> Area<T> {
        Area {
            start_x: width.start,
            end_x: width.end,
            start_y: height.start,
            end_y: height.end,
        }
    }

    pub fn height(&self) -> T {
        (self.end_y - self.start_y).max(zero())
    }

    pub fn width(&self) -> T {
        (self.end_x - self.start_x).max(zero())
    }

    pub fn size(&self) -> T {
        self.height() * self.width()
    }

    pub fn height_range(&self) -> Range<T> {
        (self.start_x)..(self.end_x)
    }

    pub fn width_range(&self) -> Range<T> {
        (self.start_y)..(self.end_y)
    }

    pub fn contains(&self, (x, y): (T, T)) -> bool {
        self.start_x <= x && x < self.end_x && self.start_y <= y && y < self.end_y
    }

    pub fn is_empty(&self) -> bool {
        self.start_x >= self.end_x || self.start_y >= self.end_y
    }

    pub fn intersection(self, other: Area<T>) -> Area<T> {
        Area {
            start_x: self.start_x.max(other.start_x),
            end_x: self.end_x.min(other.end_x),
            start_y: self.start_y.max(other.start_y),
            end_y: self.end_y.min(other.end_y),
        }
    }

    pub fn intersects(self, other: Area<T>) -> bool {
        !self.intersection(other).is_empty()
    }
}

impl<T: PrimInt> IntoIterator for Area<T> {
    type Item = (T, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntoIter<T> {
    area: Area<T>,
    x: T,
    y: T,
}

impl<T: PrimInt> IntoIter<T> {
    fn new(area: Area<T>) -> IntoIter<T> {
        IntoIter {
            area,
            x: area.start_x,
            y: area.start_y,
        }
    }
}

impl<T: PrimInt> Iterator for IntoIter<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        if self.x >= self.area.end_x || self.y == self.area.end_y {
            return None;
        }
        let p = (self.x, self.y);
        self.y = self.y + one();
        if self.y == self.area.end_y {
            self.y = self.area.start_y;
            self.x = self.x + one();
        }
        Some(p)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.x >= self.area.end_x || self.y == self.area.end_y {
            return (0, Some(0));
        }
        let sz = self.area.height() * (self.area.end_x - self.x) - (self.y - self.area.start_y);
        match <usize as NumCast>::from(sz) {
            Some(sz) => (sz, Some(sz)),
            None => (usize::MAX, None),
        }
    }
}

impl<T: PrimInt> FusedIterator for IntoIter<T> {}

impl<T: PrimInt> ExactSizeIterator for IntoIter<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_3x2() {
        let area = Area {
            start_x: 0,
            end_x: 3,
            start_y: 0,
            end_y: 2,
        };
        let mut iter = area.into_iter();
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_3x2_offset() {
        let area = Area {
            start_x: 5,
            end_x: 8,
            start_y: 4,
            end_y: 6,
        };
        let mut iter = area.into_iter();
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((5, 4)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((5, 5)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((6, 4)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((6, 5)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((7, 4)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((7, 5)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_0x2() {
        let area = Area {
            start_x: 0,
            end_x: 0,
            start_y: 0,
            end_y: 2,
        };
        let mut iter = area.into_iter();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_3x0() {
        let area = Area {
            start_x: 0,
            end_x: 3,
            start_y: 0,
            end_y: 0,
        };
        let mut iter = area.into_iter();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_0x0() {
        let area = Area {
            start_x: 0,
            end_x: 0,
            start_y: 0,
            end_y: 0,
        };
        let mut iter = area.into_iter();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
