use std::fs;
use std::sync::mpsc::channel;
use std::error::Error;
use std::collections::HashMap;
use intcode::{Intcode, AocResult};

enum HasBeam {
    Yes,
    No
}

fn has_beam(program: &Vec<i64>, x: i64, y: i64) -> AocResult<HasBeam> {
    let (sender, robot_receiver) = channel();
    let (robot_sender, receiver) = channel();
    let mut intcode = Intcode::new(program.clone(), robot_receiver, robot_sender);
    sender.send(x)?;
    sender.send(y)?;
    intcode.exec()?;
    match receiver.recv()? {
        0 => Ok(HasBeam::No),
        1 => Ok(HasBeam::Yes),
        _ => Err(Box::<dyn Error>::from("Incorrect output"))
    }
}

fn part1(program: &Vec<i64>) -> AocResult<i64> {
    let mut result = 0;
    for x in 0..50 {
        for y in 0..50 {
            if let HasBeam::Yes = has_beam(program, x, y)? {
                result += 1
            }
        }
    }
    Ok(result)
}

fn part2(program: &Vec<i64>) -> AocResult<i64> {
    const SIZE: i64 = 100;
    let mut last_y_with_beam = HashMap::new();
    let mut x = SIZE;
    loop {
        // boundaries were obtained with NN-based image recognition
        let mut lo = x / 2;
        let mut hi = x;
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            match has_beam(program, x, mid)? {
                HasBeam::Yes => lo = mid,
                HasBeam::No => hi = mid - 1,
            }
        }
        last_y_with_beam.insert(x, lo);

        if let Some(right_y) = last_y_with_beam.get(&(x - SIZE + 1)) {
            lo = 0;
            hi = x / 2;
            while lo < hi {
                let mid = (lo + hi) / 2;
                match has_beam(program, x, mid)? {
                    HasBeam::No => lo = mid + 1,
                    HasBeam::Yes => hi = mid,
                }
            }
            let left_y = lo;
            if right_y - left_y >= SIZE - 1 {
                return Ok((x - SIZE + 1) * 10000 + lo)
            }
        }
        x += 1;
    }
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part1(&program)?);
    println!("{:?}", part2(&program)?);
    Ok(())
}