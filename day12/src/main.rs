#[derive(Debug, Clone)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon { pos: [x, y, z], vel: [0, 0, 0] }
    }

    fn get_energy(&self) -> i32 {
        let a: i32 = self.pos.iter().map(|x| x.abs()).sum();
        let b: i32 = self.vel.iter().map(|x| x.abs()).sum();
        a * b
    }
}

fn part1(mut moons: Vec<Moon>) {
    for _ in 0..1000 {
        for i in 0..moons.len() {
            let (left, right) = moons.split_at_mut(i);
            let (mid, right) = right.split_at_mut(1);
            let target = &mut mid[0];
            for other in left.iter().chain(right.iter()) {
                for i in 0..3 {
                    if target.pos[i] < other.pos[i] {
                        target.vel[i] += 1
                    } else if target.pos[i] > other.pos[i] {
                        target.vel[i] -= 1
                    }
                }
            }
        }
        for moon in moons.iter_mut() {
            for i in 0..3 {
                moon.pos[i] += moon.vel[i]
            }
        }
    }
    let total_energy: i32 = moons.iter().map(|m| m.get_energy()).sum();
    println!("Total enerfy: {}", total_energy);
}

fn part2(moons: Vec<Moon>) {
    let mut periods = vec![];
    for axis in 0..3 {
        let mut axis_moons: Vec<(i32, i32)> = moons.iter().map(|m| (m.pos[axis], 0)).collect();
        let mut time = 0;
        let mut repeated = 0;
        while repeated != moons.len() {
            for i in 0..moons.len() {
                let (left, right) = axis_moons.split_at_mut(i);
                let (mid, right) = right.split_at_mut(1);
                let (pos, vel) = &mut mid[0]; 
                for &(other, _) in left.iter().chain(right.iter()) {
                    if *pos < other {
                        *vel += 1
                    } else if *pos > other {
                        *vel -= 1
                    }
                }
            }
            repeated = 0;
            for (i, (pos, vel)) in axis_moons.iter_mut().enumerate() {
                *pos += *vel;
                if *vel == 0 && *pos == moons[i].pos[axis] {
                    repeated += 1;
                }
            }
            time += 1;
        }
        periods.push(time);
    }
    println!("Orbital periods: {:?}, lcm is left as an exercise for the reader", periods);
}


fn main() {
    let moons = vec![
        Moon::new(-7, 17, -11),
        Moon::new(9, 12, 5),
        Moon::new(-9, 0, -4),
        Moon::new(4, 6, 0),
    ];
    part1(moons.clone());
    part2(moons);
}
