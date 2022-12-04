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
// TODO: Make this return an iterator instead of a Vec
pub fn points_added(p: Point, v: Vector) -> Result<Vec<Point>, NotCardinalError> {
    match v {
        Vector { x: 0, y: 0 } => Ok(Vec::new()),
        Vector { x, y: 0 } if x > 0 => Ok((1..=x).map(|i| p + Vector { x: i, y: 0 }).collect()),
        Vector { x, y: 0 } if x < 0 => Ok((x..-1).map(|i| p + Vector { x: i, y: 0 }).collect()),
        Vector { x: 0, y } if y > 0 => Ok((1..=y).map(|j| p + Vector { x: 0, y: j }).collect()),
        Vector { x: 0, y } if y < 0 => Ok((y..-1).map(|j| p + Vector { x: 0, y: j }).collect()),
        v => Err(NotCardinalError(v)),
    }
}

#[derive(Debug, Error)]
#[error("Vector is not a cardinal direction: {0:?}")]
pub struct NotCardinalError(Vector);
