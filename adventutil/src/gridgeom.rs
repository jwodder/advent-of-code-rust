use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
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

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs;
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<i32> for Vector {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item = Vector>>(iter: I) -> Vector {
        iter.fold(Vector::default(), |u, v| u + v)
    }
}

// The returned points include `p+v` but not `p` (If `v` is the zero vector,
// the collection is empty)
// Points are returned in the order they are "encountered" when going from `p`
// to `p+v`.
// TODO: Make this return an iterator instead of a Vec
pub fn points_added(p: Point, v: Vector) -> Result<Vec<Point>, NotCardinalError> {
    match v {
        Vector { x: 0, y: 0 } => Ok(Vec::new()),
        Vector { x, y: 0 } if x > 0 => Ok((1..=x).map(|i| p + Vector { x: i, y: 0 }).collect()),
        Vector { x, y: 0 } if x < 0 => Ok((1..=-x).map(|i| p - Vector { x: i, y: 0 }).collect()),
        Vector { x: 0, y } if y > 0 => Ok((1..=y).map(|j| p + Vector { x: 0, y: j }).collect()),
        Vector { x: 0, y } if y < 0 => Ok((1..=-y).map(|j| p - Vector { x: 0, y: j }).collect()),
        v => Err(NotCardinalError(v)),
    }
}

#[derive(Debug, Error)]
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
        let Point { y, x } = iter.next()?;
        let bounds = PointBounds {
            min_x: x,
            min_y: y,
            max_x: x,
            max_y: y,
        };
        Some(iter.fold(bounds, |bounds, p| bounds.with_point(p)))
    }
}
