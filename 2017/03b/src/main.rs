use adventutil::grid::{Coords, Grid, GridBounds, Padding};
use adventutil::gridgeom::{Point, PointBounds, Vector};
use adventutil::Input;

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoopIter {
    current: Point,
    direction: Vector,
    steps_left: usize,
    stride_length: usize,
}

impl LoopIter {
    fn new() -> LoopIter {
        LoopIter {
            current: Point::ORIGIN,
            direction: Vector::EAST,
            steps_left: 1,
            stride_length: 1,
        }
    }
}

impl Iterator for LoopIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let p = self.current;
        self.current += self.direction;
        self.steps_left -= 1;
        if self.steps_left < 1 {
            self.direction = self.direction.turn_left();
            if self.direction == Vector::EAST || self.direction == Vector::WEST {
                self.stride_length += 1;
            }
            self.steps_left = self.stride_length;
        }
        Some(p)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpiralCanvas(Grid<Option<u32>>);

impl SpiralCanvas {
    fn new() -> SpiralCanvas {
        SpiralCanvas(Grid::filled(GridBounds::new(5, 5), None))
    }

    fn bounds(&self) -> PointBounds {
        let layers = i32::try_from(self.0.width() / 2).unwrap();
        PointBounds {
            min_x: -layers,
            max_x: layers,
            min_y: -layers,
            max_y: layers,
        }
    }

    fn point2coords(&self, p: Point) -> Option<Coords> {
        self.bounds().coords_of_point(p, true)
    }

    fn get(&self, p: Point) -> Option<u32> {
        self.point2coords(p).and_then(|c| self.0[c])
    }

    fn set(&mut self, p: Point, n: u32) {
        if let Some(c) = self.point2coords(p) {
            self.0.set(c, Some(n));
        } else {
            self.0 = self.0.embiggened(
                Padding {
                    left: 5,
                    right: 5,
                    top: 5,
                    bottom: 5,
                },
                None,
            );
            let c = self.point2coords(p).unwrap();
            self.0.set(c, Some(n));
        }
    }
}

fn sum_neighbors(canvas: &SpiralCanvas, p: Point) -> u32 {
    let mut sum = 0;
    for xdiff in -1..=1 {
        for ydiff in -1..=1 {
            if (xdiff, ydiff) == (0, 0) {
                continue;
            }
            if let Some(n) = canvas.get(p + Vector { x: xdiff, y: ydiff }) {
                sum += n;
            }
        }
    }
    sum
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpiralIter {
    canvas: SpiralCanvas,
    inner: LoopIter,
}

impl SpiralIter {
    fn new() -> SpiralIter {
        let canvas = SpiralCanvas::new();
        let inner = LoopIter::new();
        SpiralIter { canvas, inner }
    }
}

impl Iterator for SpiralIter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let p = self.inner.next()?;
        let n = if p == Point::ORIGIN {
            1
        } else {
            sum_neighbors(&self.canvas, p)
        };
        self.canvas.set(p, n);
        Some(n)
    }
}

fn solve(input: Input) -> u32 {
    let target = input.parse::<u32>();
    SpiralIter::new().find(|&n| n > target).unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_iter() {
        let mut iter = LoopIter::new();
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0 })); // 1
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0 })); // 2
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1 })); // 3
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1 })); // 4
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1 })); // 5
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0 })); // 6
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1 })); // 7
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1 })); // 8
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1 })); // 9
        assert_eq!(iter.next(), Some(Point { x: 2, y: -1 })); // 10
        assert_eq!(iter.next(), Some(Point { x: 2, y: 0 })); // 11
        assert_eq!(iter.next(), Some(Point { x: 2, y: 1 })); // 12
        assert_eq!(iter.next(), Some(Point { x: 2, y: 2 })); // 13
        assert_eq!(iter.next(), Some(Point { x: 1, y: 2 })); // 14
        assert_eq!(iter.next(), Some(Point { x: 0, y: 2 })); // 15
        assert_eq!(iter.next(), Some(Point { x: -1, y: 2 })); // 16
        assert_eq!(iter.next(), Some(Point { x: -2, y: 2 })); // 17
        assert_eq!(iter.next(), Some(Point { x: -2, y: 1 })); // 18
        assert_eq!(iter.next(), Some(Point { x: -2, y: 0 })); // 19
        assert_eq!(iter.next(), Some(Point { x: -2, y: -1 })); // 20
        assert_eq!(iter.next(), Some(Point { x: -2, y: -2 })); // 21
        assert_eq!(iter.next(), Some(Point { x: -1, y: -2 })); // 22
        assert_eq!(iter.next(), Some(Point { x: 0, y: -2 })); // 23
    }

    #[test]
    fn test_spiral_iter() {
        let mut iter = SpiralIter::new();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(23));
        assert_eq!(iter.next(), Some(25));
        assert_eq!(iter.next(), Some(26));
        assert_eq!(iter.next(), Some(54));
        assert_eq!(iter.next(), Some(57));
        assert_eq!(iter.next(), Some(59));
        assert_eq!(iter.next(), Some(122));
        assert_eq!(iter.next(), Some(133));
        assert_eq!(iter.next(), Some(142));
        assert_eq!(iter.next(), Some(147));
        assert_eq!(iter.next(), Some(304));
        assert_eq!(iter.next(), Some(330));
        assert_eq!(iter.next(), Some(351));
        assert_eq!(iter.next(), Some(362));
        assert_eq!(iter.next(), Some(747));
        assert_eq!(iter.next(), Some(806));
    }
}
