use std::fs;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use intcode::{Intcode, AocResult};


fn part1(program: &Vec<i64>) -> i32 {
    let (_, robot_receiver) = channel();
    let (robot_sender, receiver) = channel();
    let mut robot = Intcode::new(program.clone(), robot_receiver, robot_sender);
    thread::spawn(move || robot.exec().unwrap()).join().unwrap();

    let received: Vec<_> = receiver.iter().collect();
    let mut blocks_count = 0;
    for obj in received.chunks(3) {
        if obj[2] == 2 {
            blocks_count += 1;
        }
    }
    blocks_count
}

fn get_initial_paddle(receiver: &Receiver<i64>) -> AocResult<i64> {
    loop {
        let x = receiver.recv()?;
        receiver.recv()?;
        let tile = receiver.recv()?;
        if tile == 3 {
            return Ok(x);
        }
    }
}

fn part2(mut program: Vec<i64>) -> AocResult<i64> {
    let (sender, robot_receiver) = channel();
    let (robot_sender, receiver) = channel();

    let mut result = 0;
    program[0] = 2;
    sender.send(0)?;

    let mut robot = Intcode::new(program, robot_receiver, robot_sender);
    thread::spawn(move || robot.exec().unwrap());

    let mut paddle = get_initial_paddle(&receiver)?;
    loop {
        let x = match receiver.recv() {
            Ok(x) => x,
            Err(_) => return Ok(result)
        };
        receiver.recv()?;
        let tile = receiver.recv()?;
        if x == -1 && tile > result {
            result = tile;
        } else if tile == 4 {
            let msg = if paddle > x {
                -1
            } else if paddle < x {
                1
            } else {
                0
            };
            paddle += msg;
            sender.send(msg)?;
        }
    }
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
    println!("{}", part1(&program));
    println!("{}", part2(program)?);
    Ok(())
}
