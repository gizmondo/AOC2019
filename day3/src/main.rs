use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;
use regex::Regex;

type AocResult<T> = std::result::Result<T, Box<Error>>;

struct Point {
    x: i32,
    y: i32
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical
}

struct Interval {
    fixed_coord: i32,
    first_coord: i32,
    second_coord: i32,
    orientation: Orientation
}

impl Interval {
    fn find_intersection(&self, other: &Interval) -> Option<Point> {
        if self.orientation != other.orientation &&
            other.get_range().contains(&self.fixed_coord) &&
            self.get_range().contains(&other.fixed_coord)
        {
            let point = match self.orientation {
                Orientation::Horizontal => Point {x: self.fixed_coord, y: other.fixed_coord},
                Orientation::Vertical => Point {x: other.fixed_coord, y: self.fixed_coord}
            };
            Some(point)
        } else {
            None
        }
    }

    fn get_range(&self) -> RangeInclusive<i32> {
        if self.first_coord < self.second_coord {
            self.first_coord ..= self.second_coord
        } else {
            self.second_coord ..= self.first_coord
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
        let re = Regex::new(r"([UDLR])(\d+)")?;
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
                        first_coord: cur_y,
                        second_coord: next_y,
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
                        first_coord: cur_x,
                        second_coord: next_x,
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

    fn find_all_intersections(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
        // O(first.len * second.len).
        // Could be improved to O(first.len * ln(second.len)) with bisect and interval tree.
        let mut result: Vec<Point> = Vec::new();
        for first in &wire1.intervals {
            for second in &wire2.intervals {
                match Interval::find_intersection(&first, &second) {
                    Some(i) => result.push(i),
                    None => {}
                }
            }
        }
        result
    }
}

fn part1(wire1: &Wire, wire2: &Wire) -> AocResult<i32> {
    let mut result = None;
    for point in Wire::find_all_intersections(wire1, wire2) {
        let distance = point.x.abs() + point.y.abs();
        if distance == 0 {
            // We ignore intersection in the initial point
            continue;
        }
        if let Some(min_distance) = result {
            if min_distance > distance {
                result = Some(distance);
            }
        } else {
            result = Some(distance)
        }
    }
    result.ok_or(Box::from("Intersection not found"))
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let wire1 = Wire::new(lines[0])?;
    let wire2 = Wire::new(lines[1])?;
    println!("{}", part1(&wire1, &wire2)?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> AocResult<()> {
        assert_eq!(
            part1(
                &Wire::new("R8,U5,L5,D3")?,
                &Wire::new("U7,R6,D4,L4")?
            )?,
            6
        );
        assert_eq!(
            part1(
                &Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72")?,
                &Wire::new("U62,R66,U55,R34,D71,R55,D58,R83")?
            )?,
            159
        );
        assert_eq!(
            part1(
                &Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?,
                &Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?
            )?,
            135
        );
        Ok(())
    }
}