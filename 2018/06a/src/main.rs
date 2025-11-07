// Strategy: Identify the coordinates with infinite area by determining the
// bounding box (inclusive) for the coordinates as a whole and then shrinking
// it as follows:
//
// - For each side of the bounding box:
//     - Loop:
//         - Move the side one unit inwards.
//         - If there exists a point along the side whose closest coordinates
//           are all within the bounds, exit the loop.
//
// Afterwards, the coordinates outside the bounds are the ones with infinite
// area, and the bounding box contains all points whose nearest coordinate is
// one with finite area.
use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::gridgeom::{Point, PointBounds};

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(',').unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    let y = y.trim().parse::<i32>().unwrap();
    Point { x, y }
}

fn solve(input: Input) -> u64 {
    let coords = input.lines().map(|s| parse_point(&s)).collect::<Vec<_>>();
    let mut bounds = PointBounds::for_points(coords.iter().copied()).unwrap();

    'minx: loop {
        bounds.min_x += 1;
        let mut points_outside = Vec::new();
        let mut points_inside = Vec::new();
        for &p in &coords {
            if bounds.contains(p) {
                points_inside.push(p);
            } else {
                points_outside.push(p);
            }
        }
        let x = bounds.min_x;
        for y in bounds.min_y..=bounds.max_y {
            let p = Point { x, y };
            if let Some(closest_dist_outside) = points_outside
                .iter()
                .map(|&p2| (p - p2).taxicab_len())
                .min()
            {
                let closest_dist_inside = points_inside
                    .iter()
                    .map(|&p2| (p - p2).taxicab_len())
                    .min()
                    .unwrap();
                if closest_dist_inside < closest_dist_outside {
                    break 'minx;
                }
            }
        }
    }

    'maxx: loop {
        bounds.max_x -= 1;
        let mut points_outside = Vec::new();
        let mut points_inside = Vec::new();
        for &p in &coords {
            if bounds.contains(p) {
                points_inside.push(p);
            } else {
                points_outside.push(p);
            }
        }
        let x = bounds.max_x;
        for y in bounds.min_y..=bounds.max_y {
            let p = Point { x, y };
            if let Some(closest_dist_outside) = points_outside
                .iter()
                .map(|&p2| (p - p2).taxicab_len())
                .min()
            {
                let closest_dist_inside = points_inside
                    .iter()
                    .map(|&p2| (p - p2).taxicab_len())
                    .min()
                    .unwrap();
                if closest_dist_inside < closest_dist_outside {
                    break 'maxx;
                }
            }
        }
    }

    'miny: loop {
        bounds.min_y += 1;
        let mut points_outside = Vec::new();
        let mut points_inside = Vec::new();
        for &p in &coords {
            if bounds.contains(p) {
                points_inside.push(p);
            } else {
                points_outside.push(p);
            }
        }
        let y = bounds.min_y;
        for x in bounds.min_x..=bounds.max_x {
            let p = Point { x, y };
            if let Some(closest_dist_outside) = points_outside
                .iter()
                .map(|&p2| (p - p2).taxicab_len())
                .min()
            {
                let closest_dist_inside = points_inside
                    .iter()
                    .map(|&p2| (p - p2).taxicab_len())
                    .min()
                    .unwrap();
                if closest_dist_inside < closest_dist_outside {
                    break 'miny;
                }
            }
        }
    }

    'maxy: loop {
        bounds.max_y -= 1;
        let mut points_outside = Vec::new();
        let mut points_inside = Vec::new();
        for &p in &coords {
            if bounds.contains(p) {
                points_inside.push(p);
            } else {
                points_outside.push(p);
            }
        }
        let y = bounds.max_y;
        for x in bounds.min_x..=bounds.max_x {
            let p = Point { x, y };
            if let Some(closest_dist_outside) = points_outside
                .iter()
                .map(|&p2| (p - p2).taxicab_len())
                .min()
            {
                let closest_dist_inside = points_inside
                    .iter()
                    .map(|&p2| (p - p2).taxicab_len())
                    .min()
                    .unwrap();
                if closest_dist_inside < closest_dist_outside {
                    break 'maxy;
                }
            }
        }
    }

    let mut counter = Counter::new();
    for y in bounds.min_y..=bounds.max_y {
        for x in bounds.min_x..=bounds.max_x {
            let p = Point { x, y };
            let mut min_dist = None;
            let mut nearest = Vec::new();
            for &c in &coords {
                let d = (p - c).taxicab_len();
                if min_dist.is_none_or(|md| d < md) {
                    min_dist = Some(d);
                    nearest.clear();
                    nearest.push(c);
                } else if min_dist == Some(d) {
                    nearest.push(c);
                }
            }
            if nearest.len() == 1 {
                counter.add(nearest.pop().unwrap());
            }
        }
    }
    counter
        .into_iter()
        .filter_map(|(c, qty)| bounds.contains(c).then_some(qty))
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n");
        assert_eq!(solve(input), 17);
    }
}
