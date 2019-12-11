use std::fs::read_to_string;
use std::error::Error;
use std::collections::HashMap;

type AocResult<T> = std::result::Result<T, Box<dyn Error>>;

fn euclid(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        euclid(b, a % &b)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Inclination {
    dx: i32,
    dy: i32
}

impl Inclination {
    fn new(a: &Point, b: &Point) -> Inclination {
        let init_dx = a.x - b.x;
        let init_dy = a.y - b.y;
        let gcd = euclid(init_dx, init_dy);
        let dx = init_dx / gcd;
        let dy = init_dy / gcd;
        Inclination { dx, dy }
    }

    fn get_angle(&self) -> f64 {
        let mut atan = -(self.dx as f64).atan2(self.dy as f64);
        if atan < 0.0 {
            atan += 2.0 * std::f64::consts::PI;
        }
        atan
    }
}

fn parse(input: &str) -> Vec<Point> {
    input.trim().split_whitespace().enumerate().map(
        |(y, row)| {
            row.chars().enumerate().filter_map(
                move |(x, ch)| {
                    match ch {
                        '#' => Some(Point { x: x as i32, y: y as i32 }),
                        _ => None
                    }
                }
            )
        }
    ).flatten().collect()
}

struct Station {
    center: Point,
    lines: HashMap<Inclination, Vec<Point>>
}

fn find_best_station(input: &str) -> AocResult<Station> {
    let mut result = None;
    let points: Vec<Point> = parse(input);
    for center in points.to_vec() {
        let mut lines = HashMap::new();
        for other_point in points.to_vec() {
            if center == other_point {
                continue;
            }
            let entry = lines.entry(Inclination::new(&center, &other_point)).or_insert(vec![]);
            entry.push(other_point)
        }
        let station = Station { center, lines };
        match &result {
            None => result = Some(station),
            Some(other) => if other.lines.len() < station.lines.len() {
                result = Some(station)
            }
        }
    }
    result.ok_or(Box::<dyn Error>::from("No asteroids"))
}

struct Ring<T> {
    items: Vec<Vec<T>>,
    next: Vec<Option<usize>>,
    idx: usize
}

impl<T> Ring<T> {
    fn new(items: Vec<Vec<T>>) -> Ring<T> {
        let len = items.len();
        let mut next: Vec<Option<usize>> = (1..len).map(|x| Some(x)).collect();
        next.push(Some(0));
        Ring { items, next, idx: len - 1 }
    }
}

impl<T> Iterator for Ring<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.next[self.idx].map(|idx| {
            let v = &mut self.items[idx];
            let result = v.pop().unwrap();  // self.points must not contain empty arrays
            if v.is_empty() {
                self.next[self.idx] = self.next[idx];
                self.next[idx] = None;
            } else {
                self.idx = idx;
            }
            result
        })
    }
}

fn nth_to_vaporize(mut station: Station, n: i32) -> AocResult<i32> {
    // sort by distance
    let center = station.center.clone();
    for points in station.lines.values_mut() {
        points.sort_by_key(|p| -p.dist(&center));
    }

    // sort by angle
    let mut inclinations: Vec<_> = station.lines.keys().collect();
    inclinations.sort_by(|a, b| {
        let angle1 = &a.get_angle();
        let angle2 = &b.get_angle();
        angle1.partial_cmp(angle2).unwrap()
    });

    let mut ring = Ring::new(
        inclinations.into_iter().map(|incl| station.lines.get(incl).unwrap().to_vec()).collect()
    );

    let mut nth: Option<Point> = None;
    for _ in 0..n {
        nth = ring.next();
    }
    nth.map(|p| 100 * p.x + p.y).ok_or(Box::<dyn Error>::from("Illegal n"))
}

fn main() -> AocResult<()> {
    let input = read_to_string("input.txt")?;
    let best_station = find_best_station(&input)?;
    println!("Part1: {}", best_station.lines.len());
    println!("Part2: {}", nth_to_vaporize(best_station, 200)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring() -> AocResult<()> {
        let ring: Ring<i32> = Ring::new(vec![vec![1], vec![6, 4, 2], vec![5, 3]]);
        assert_eq!(ring.into_iter().collect::<Vec<_>>(), (1..=6).collect::<Vec<_>>());
        Ok(())
    }

    #[test]
    fn test_task() -> AocResult<()> {
        let input = "\
            .#..##.###...#######\n\
            ##.############..##.\n\
            .#.######.########.#\n\
            .###.#######.####.#.\n\
            #####.##.#.##.###.##\n\
            ..#####..#.#########\n\
            ####################\n\
            #.####....###.#.#.##\n\
            ##.#################\n\
            #####.##.###..####..\n\
            ..######..##.#######\n\
            ####.##.####...##..#\n\
            .#####..#.######.###\n\
            ##...#.##########...\n\
            #.##########.#######\n\
            .####.#.###.###.#.##\n\
            ....##.##.###..#####\n\
            .#.#.###########.###\n\
            #.#.#.#####.####.###\n\
            ###.##.####.##.#..##\
        ";
        let best_station = find_best_station(&input)?;
        assert_eq!(best_station.lines.len(), 210);
        assert_eq!(nth_to_vaporize(best_station, 200)?, 802);
        Ok(())
    }
}
