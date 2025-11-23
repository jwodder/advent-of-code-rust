use super::grid::Coords;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub const NORTH: Vector = Vector { x: 0, y: 1 };
    pub const SOUTH: Vector = Vector { x: 0, y: -1 };
    pub const EAST: Vector = Vector { x: 1, y: 0 };
    pub const WEST: Vector = Vector { x: -1, y: 0 };

    // Returns the vector after rotating 90 degrees to the
    // left/counterclockwise
    pub fn turn_left(&self) -> Vector {
        // Rotation matrix: [[0, -1], [1, 0]]
        Vector {
            x: -self.y,
            y: self.x,
        }
    }

    // Returns the vector after rotating 90 degrees to the right/clockwise
    pub fn turn_right(&self) -> Vector {
        // Rotation matrix: [[0, 1], [-1, 0]]
        Vector {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn taxicab_len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs;
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs;
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::MulAssign<i32> for Vector {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl std::iter::Sum for Vector {
    fn sum<I: Iterator<Item = Vector>>(iter: I) -> Vector {
        iter.fold(Vector::default(), |u, v| u + v)
    }
}

// The yielded points include `p+v` but not `p` (If `v` is the zero vector, the
// iterator is empty)
// Points are yielded in the order they are "encountered" when going from `p`
// to `p+v`.
pub fn points_added(p: Point, v: Vector) -> Result<PointsAdded, NotCardinalError> {
    let (unit, offset) = match v {
        Vector { x: 0, y: 0 } => (Vector::NORTH, 0),
        Vector { x, y: 0 } if x > 0 => (Vector::EAST, x),
        Vector { x, y: 0 } if x < 0 => (Vector::WEST, x),
        Vector { x: 0, y } if y > 0 => (Vector::NORTH, y),
        Vector { x: 0, y } if y < 0 => (Vector::SOUTH, y),
        v => return Err(NotCardinalError(v)),
    };
    Ok(PointsAdded {
        p,
        unit,
        inner: 0..offset.abs_diff(0),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointsAdded {
    p: Point,
    unit: Vector,
    inner: std::ops::Range<u32>,
}

impl Iterator for PointsAdded {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let _ = self.inner.next()?;
        self.p += self.unit;
        Some(self.p)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl std::iter::FusedIterator for PointsAdded {}

impl ExactSizeIterator for PointsAdded {}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("vector is not a cardinal direction: {0:?}")]
pub struct NotCardinalError(Vector);

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct PointBounds {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl PointBounds {
    pub fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }

    pub fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    // Treats the Point coordinate system as being oriented in a normal
    // Cartesian fashion, not in a graphics-display fashion
    pub fn ulcorner(&self) -> Point {
        Point {
            x: self.min_x,
            y: self.max_y,
        }
    }

    // If `downwards` is true, the Y-axis of the Coords will be treated as
    // starting at the top of the PointBounds and increasing downwards;
    // otherwise, it starts at the bottom of the PointBounds and increases
    // upwards.
    pub fn at_coords(&self, c: Coords, downwards: bool) -> Point {
        let x = self.min_x + i32::try_from(c.x).unwrap();
        let y = if downwards {
            self.max_y - i32::try_from(c.y).unwrap()
        } else {
            self.min_y + i32::try_from(c.y).unwrap()
        };
        Point { x, y }
    }

    // If `downwards` is true, the Y-axis of the Coords will be treated as
    // starting at the top of the PointBounds and increasing downwards;
    // otherwise, it starts at the bottom of the PointBounds and increases
    // upwards.
    pub fn coords_of_point(&self, p: Point, downwards: bool) -> Option<Coords> {
        if !self.contains(p) {
            return None;
        }
        let x = usize::try_from(p.x - self.min_x).unwrap();
        let y = if downwards {
            usize::try_from(self.max_y - p.y).unwrap()
        } else {
            usize::try_from(p.y - self.min_y).unwrap()
        };
        Some(Coords { x, y })
    }

    pub fn for_point(Point { y, x }: Point) -> PointBounds {
        PointBounds {
            min_x: x,
            min_y: y,
            max_x: x,
            max_y: y,
        }
    }

    pub fn with_point(self, point: Point) -> PointBounds {
        PointBounds {
            min_x: self.min_x.min(point.x),
            min_y: self.min_y.min(point.y),
            max_x: self.max_x.max(point.x),
            max_y: self.max_y.max(point.y),
        }
    }

    pub fn for_points<I: IntoIterator<Item = Point>>(iter: I) -> Option<PointBounds> {
        let mut iter = iter.into_iter();
        let bounds = PointBounds::for_point(iter.next()?);
        Some(iter.fold(bounds, PointBounds::with_point))
    }

    pub fn contains(self, point: Point) -> bool {
        (self.min_x..=self.max_x).contains(&point.x) && (self.min_y..=self.max_y).contains(&point.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_added_zero_vector() {
        let mut iter = points_added(Point { x: 23, y: 42 }, Vector { x: 0, y: 0 }).unwrap();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn points_added_northwards() {
        let mut iter = points_added(Point { x: 23, y: 42 }, Vector { x: 0, y: 7 }).unwrap();
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 43 }));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 44 }));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 45 }));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 46 }));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 47 }));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 48 }));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(Point { x: 23, y: 49 }));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn points_added_westwards() {
        let mut iter = points_added(Point { x: 23, y: 42 }, Vector { x: -7, y: 0 }).unwrap();
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.next(), Some(Point { x: 22, y: 42 }));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(Point { x: 21, y: 42 }));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some(Point { x: 20, y: 42 }));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some(Point { x: 19, y: 42 }));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(Point { x: 18, y: 42 }));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(Point { x: 17, y: 42 }));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(Point { x: 16, y: 42 }));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn points_added_diagonal() {
        assert_eq!(
            points_added(Point { x: 23, y: 42 }, Vector { x: -7, y: 5 }),
            Err(NotCardinalError(Vector { x: -7, y: 5 }))
        );
    }
}
