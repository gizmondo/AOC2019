use std::fs;
use std::sync::mpsc::channel;
use std::thread;
use std::error::Error;
use std::collections::HashMap;
use itertools::Itertools;
use intcode::{Intcode, AocResult};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

fn paint(program: &Vec<i64>, white: &mut HashMap<(i32, i32), bool>) -> AocResult<()> {
    let (sender, robot_receiver) = channel();
    let (robot_sender, receiver) = channel();
    let mut robot = Intcode::new(program.clone(), robot_receiver, robot_sender);
    thread::spawn(move || robot.exec().unwrap());

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir_idx: usize = 0;
    let mut pos = (0, 0);

    loop {
        let e = white.entry(pos.clone()).or_insert(false);
        let msg = match &e {
            true => 1,
            false => 0
        };
        sender.send(msg)?;
        *e = match receiver.recv() {
            Ok(c) if c == 0 => false,
            Ok(c) if c == 1 => true,
            Ok(c) => return err!("Illegal color: {}", c),
            Err(_) => return Ok(())
        };

        dir_idx = match receiver.recv() {
            Ok(c) if c == 0 => match dir_idx {
                0 => 3,
                v => v - 1
            },
            Ok(c) if c == 1 => (dir_idx + 1) % 4,
            Ok(c) => return err!("Illegal rotation: {}", c),
            Err(_) => return Ok(())
        };
        let direction = directions[dir_idx];
        pos.0 += direction.0;
        pos.1 += direction.1;
    }
}

fn part1(program: &Vec<i64>) -> AocResult<()> {
    let mut white = HashMap::new();
    paint(&program, &mut white)?;
    println!("{}", white.len());
    Ok(())
}


fn part2(program: &Vec<i64>) -> AocResult<()> {
    let mut white = HashMap::new();
    white.insert((0, 0), true);
    paint(&program, &mut white)?;
    let (x_low, x_high) = white.keys().map(|(x, _)| x).minmax().into_option().unwrap();
    let (y_low, y_high) = white.keys().map(|(_, y)| y).minmax().into_option().unwrap();
    for y in (*y_low..=*y_high).rev() {
        let mut s = String::new();
        for x in *x_low..=*x_high {
            let ch = match white.get(&(x, y)) {
                Some(true) => '0',
                _ => ' '
            };
            s.push(ch)
        }
        println!("{}", s);
    }
    Ok(())
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
    part1(&program)?;
    part2(&program)?;
    Ok(())
}