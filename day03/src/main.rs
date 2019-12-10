use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;
use regex::Regex;

type AocResult<T> = std::result::Result<T, Box<Error>>;

struct Intersection<'a> {
    x: i32,
    y: i32,
    horizontal: &'a Interval,
    vertical: &'a Interval
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
    start_time: i32,
    orientation: Orientation
}

impl Interval {
    fn find_intersection<'a>(&'a self, other: &'a Interval) -> Option<Intersection<'a>> {
        if self.orientation != other.orientation &&
            other.get_range().contains(&self.fixed_coord) &&
            self.get_range().contains(&other.fixed_coord)
        {
            let point = match self.orientation {
                Orientation::Horizontal => Intersection {
                    x: other.fixed_coord,
                    y: self.fixed_coord,
                    horizontal: &self,
                    vertical: &other
                },
                Orientation::Vertical => Intersection {
                    x: self.fixed_coord,
                    y: other.fixed_coord,
                    horizontal: &other,
                    vertical: &self
                }
            };
            if self.start_time != 0 || other.start_time != 0 {
                return Some(point);
            }
        }
        None
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
        let mut time = 0;
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
                        start_time: time,
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
                        start_time: time,
                        orientation: Orientation::Horizontal,
                    };
                    cur_x = next_x;
                },
                _ => unreachable!()
            }
            time += step;
            wire.intervals.push(interval);
        }
        Ok(wire)
    }

    fn find_all_intersections<'a>(wire1: &'a Wire, wire2: &'a Wire) -> Vec<Intersection<'a>> {
        // O(first.len * second.len).
        // Could be improved to O(first.len * ln(second.len)) with bisect and interval tree?
        let mut result: Vec<Intersection> = Vec::new();
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

fn part1(intersections: &Vec<Intersection>) -> i32 {
    intersections.iter().map(|i| i.x.abs() + i.y.abs()).min().unwrap()
}

fn part2(intersections: &Vec<Intersection>) -> i32 {
    intersections.iter().map(
        |i| i.horizontal.start_time + i.vertical.start_time +
            (i.horizontal.first_coord - i.x).abs() + (i.vertical.first_coord - i.y).abs()
    ).min().unwrap()
}

fn solve(wire1: &Wire, wire2: &Wire) -> AocResult<(i32, i32)> {
    let intersections = Wire::find_all_intersections(wire1, wire2);
    if intersections.len() == 0 {
        Err(Box::from("Intersection not found"))
    } else {
        Ok((part1(&intersections), part2(&intersections)))
    }

}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let wire1 = Wire::new(lines[0])?;
    let wire2 = Wire::new(lines[1])?;
    println!("{:?}", solve(&wire1, &wire2)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() -> AocResult<()> {
        assert_eq!(
            solve(
                &Wire::new("R8,U5,L5,D3")?,
                &Wire::new("U7,R6,D4,L4")?
            )?,
            (6, 30)
        );
        assert_eq!(
            solve(
                &Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72")?,
                &Wire::new("U62,R66,U55,R34,D71,R55,D58,R83")?
            )?,
            (159, 610)
        );
        assert_eq!(
            solve(
                &Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?,
                &Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?
            )?,
            (135, 410)
        );
        Ok(())
    }
}