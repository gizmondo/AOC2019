use std::error::Error;
use std::fs;
use std::cmp;
use regex::Regex;

type AocResult<T> = std::result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

struct Point {
    x: i32,
    y: i32
}

#[derive(PartialEq, Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical
}

struct Interval {
    fixed_coord: i32,
    varied_coord_first: i32,
    varied_coord_second: i32,
    orientation: Orientation
}

enum Intersection {
    P(Point),
    I(Interval)
}

impl Interval {
    fn find_intersection(a: &Interval, b: &Interval) -> Option<Intersection> {
        if a.orientation == b.orientation {
            if a.fixed_coord != b.fixed_coord {
                return None;
            }
            let first = cmp::max(&a.varied_coord_first, &b.varied_coord_first);
            let second = cmp::min(&a.varied_coord_second, &b.varied_coord_second);
            if second < first {
                None
            } else if second == first {
                let point = Interval::get_point(a.orientation, a.fixed_coord, *first);
                Some(Intersection::P(point))
            } else {
                let interval = Interval {
                    orientation: a.orientation,
                    fixed_coord: a.fixed_coord,
                    varied_coord_first: *first,
                    varied_coord_second: *second
                };
                Some(Intersection::I(interval))
            }
        } else {
            if a.fixed_coord >= b.varied_coord_first &&
                a.fixed_coord <= b.varied_coord_second &&
                b.fixed_coord >= a.varied_coord_first &&
                b.fixed_coord <= b.varied_coord_second {
                let point = Interval::get_point(a.orientation, a.fixed_coord, b.fixed_coord);
                Some(Intersection::P(point))
            } else {
                None
            }
        }
    }

    fn find_all_intersections(wire1: &Vec<Interval>, wire2: &Vec<Interval>) -> Vec<Intersection> {
        // O(first.len * second.len).
        // Could be improved to O(first.len * ln(second.len)) with bisect and interval tree.
        let mut result: Vec<Intersection> = Vec::new();
        for first in wire1 {
            for second in wire2 {
                match Interval::find_intersection(first, second) {
                    Some(i) => result.push(i),
                    None => {}
                }
            }
        }
        result
    }

    fn get_point(orientation: Orientation, fixed_coord: i32, varied_coord: i32) -> Point {
        match orientation {
            Orientation::Horizontal => Point {x: varied_coord, y: fixed_coord},
            Orientation::Vertical => Point {x: fixed_coord, y: varied_coord}
        }
    }
}

struct Wire {
    intervals: Vec<Interval>
}

impl Wire {
    fn new(input: &str) -> AocResult<Wire> {
        let mut wire = Wire {intervals: Vec::new()};
        let mut cur_x = 0;
        let mut cur_y = 0;
        let re = Regex::new(r"^([UDLR])(\d+)$")?;
        for cap in re.captures_iter(input) {
            let direction: &str = &cap[1];
            let step: i32 = cap[2].parse()?;
            let interval;
            match direction {
                "U" | "D" => {
                    let next_y;
                    if let "U" = direction {
                        next_y = cur_y + step;
                    } else {
                        next_y = cur_y - step;
                    }
                    interval = Interval {
                        fixed_coord: cur_x,
                        varied_coord_first: cmp::min(cur_y, next_y),
                        varied_coord_second: cmp::max(cur_y, next_y),
                        orientation: Orientation::Vertical,
                    };
                    cur_y = next_y;
                },
                "L" | "R" => {
                    let next_x;
                    if let "R" = direction {
                        next_x = cur_x + step;
                    } else {
                        next_x = cur_x - step;
                    }
                    interval = Interval {
                        fixed_coord: cur_y,
                        varied_coord_first: cmp::min(cur_x, next_x),
                        varied_coord_second: cmp::max(cur_x, next_x),
                        orientation: Orientation::Horizontal,
                    };
                    cur_x = next_x;
                },
                _ => unreachable!()
            }
            wire.intervals.push(interval);
        }
        Ok(wire)
    }
}

fn main() {
}
