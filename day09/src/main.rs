use std::fs;
use std::sync::mpsc::channel;
use intcode::{Intcode, AocResult};

fn part1(program: &Vec<i64>) -> AocResult<i64> {
    let (sender, receiver) = channel();
    sender.send(1)?;
    let mut intcode = Intcode::new(program.clone(), receiver, sender);
    intcode.exec()?;
    Ok(intcode.input.recv()?)
}

fn part2(program: &Vec<i64>) -> AocResult<i64> {
    let (sender, receiver) = channel();
    sender.send(2)?;
    let mut intcode = Intcode::new(program.clone(), receiver, sender);
    intcode.exec()?;
    Ok(intcode.input.recv()?)
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part1(&program)?);
    println!("{:?}", part2(&program)?);
    Ok(())
}