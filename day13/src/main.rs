use std::fs;
use std::thread;
use std::error::Error;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, channel};

pub type AocResult<T> = std::result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

pub struct Intcode {
    program: Vec<i64>,
    heap: HashMap<usize, i64>,
    pc: usize,
    relative_offset: i64,
    pub output: Sender<i64>,
    halted: bool,

    // cheating
    collect: bool,
    collected: Vec<i64>,
    paddle_x: i64,
    ball_x_addr: i64
}

#[derive(Debug)]
enum Mode { Address, Immediate, Relative }

#[derive(Debug)]
enum Op { Add, Mul, Input, Output, Jit, Jif, Lt, Eq, Rel, Halt }

#[derive(Debug)]
struct Arg {
    value: i64,
    mode: Mode
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    args: Vec<Arg>
}

impl Intcode {
    pub fn new(program: Vec<i64>, output: Sender<i64>, collect: bool) -> Intcode {
        Intcode {
            program, output,
            heap: HashMap::new(), pc: 0, relative_offset: 0, halted: false,
            collect, collected: Vec::new(), paddle_x: -1, ball_x_addr: -1
        }
    }

    fn parse(&self) -> AocResult<Instruction> {
        let opcode = self.program[self.pc];
        let (op, args_num) = match opcode % 100 {
            1 => (Op::Add, 3),
            2 => (Op::Mul, 3),
            3 => (Op::Input, 1),
            4 => (Op::Output, 1),
            5 => (Op::Jit, 2),
            6 => (Op::Jif, 2),
            7 => (Op::Lt, 3),
            8 => (Op::Eq, 3),
            9 => (Op::Rel, 1),
            99 => (Op::Halt, 0),
            illegal => return err!("Illegal opcode {} at position {}", illegal, self.pc)
        };
        let mut modes = opcode / 100;
        let mut args: Vec<Arg> = Vec::new();
        for arg_num in 0..args_num {
            let value = self.program[self.pc + arg_num + 1];
            let mode = match modes % 10 {
                0 => Mode::Address,
                1 => Mode::Immediate,
                2 => Mode::Relative,
                illegal => return err!("Illegal mode '{}'", illegal)
            };
            args.push(Arg {value, mode});
            modes = modes / 10;
        }
        Ok(Instruction {op, args})
    }

    pub fn exec(&mut self) -> AocResult<()> {
        while self.pc < self.program.len() && self.halted != true {
            let instruction = self.parse()?;
            let len = instruction.args.len();
            match instruction.op {
                Op::Halt => self.halt(),
                Op::Jit => self.jump(&instruction.args, i64::ne),
                Op::Jif => self.jump(&instruction.args, i64::eq),
                _ => {
                    self.pc += len + 1;
                    match instruction.op {
                        Op::Add => self.add(&instruction.args),
                        Op::Mul => self.mul(&instruction.args),
                        Op::Input => self.input(&instruction.args)?,
                        Op::Output => self.output(&instruction.args)?,
                        Op::Lt => self.cmp(&instruction.args, i64::lt),
                        Op::Eq => self.cmp(&instruction.args, i64::eq),
                        Op::Rel => self.set_relative_offset(&instruction.args),
                        _ => unreachable!()
                    }
                }
            }
        }
        Ok(())
    }

    fn get_value(&mut self, arg: &Arg) -> i64 {
        match arg.mode {
            Mode::Immediate => arg.value,
            Mode::Address | Mode::Relative => *self.get_cell(arg)
        }
    }

    fn get_cell(&mut self, arg: &Arg) -> &mut i64 {
        let address = match arg.mode {
            Mode::Address => arg.value,
            Mode::Relative => arg.value + self.relative_offset,
            Mode::Immediate => panic!("Not an address")  // FIXME: remove the panic
        };
        let offset = address - self.program.len() as i64;
        if offset < 0 {
            &mut self.program[address as usize]
        } else {
            self.heap.entry(offset as usize).or_insert(0)
        }
    }

    fn add(&mut self, args: &Vec<Arg>) {
        let value0 = self.get_value(&args[0]);
        let value1 = self.get_value(&args[1]);
        let cell = self.get_cell(&args[2]);
        *cell = value0 + value1;
    }

    fn mul(&mut self, args: &Vec<Arg>) {
        let value0 = self.get_value(&args[0]);
        let value1 = self.get_value(&args[1]);
        let cell = self.get_cell(&args[2]);
        *cell = value0 * value1;
    }

    fn input(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        if self.paddle_x == -1 {
            self.collect = false;
            for obj in self.collected.chunks(3) {
                if obj[2] == 3 {
                    // paddle
                    self.paddle_x = obj[0]
                } else if obj[2] == 4 {
                    // ball
                    for (i, (&x, &y)) in self.program
                        .iter()
                        .zip(self.program.split_first().unwrap().1)
                        .enumerate() {
                        if x == obj[0] && y == obj[1] {
                            self.ball_x_addr = i as i64;
                            break;
                        }
                    }
                }
            }
        }

        let ball_x = self.program[self.ball_x_addr as usize];
        let value = if self.paddle_x < ball_x {
            1
        } else if self.paddle_x > ball_x {
            -1
        } else {
            0
        };
        self.paddle_x += value;
        let cell = self.get_cell(&args[0]);
        *cell = value;
        Ok(())
    }

    fn output(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        let value = self.get_value(&args[0]);
        if !self.collect {
            self.output.send(value)?;
        } else {
            self.collected.push(value);
        }
        Ok(())
    }

    fn jump<F>(&mut self, args: &Vec<Arg>, cond: F)
        where F: Fn(&i64, &i64) -> bool
    {
        if cond(&self.get_value(&args[0]), &0) {
            self.pc = self.get_value(&args[1]) as usize;
        } else {
            self.pc += args.len() + 1;
        }
    }

    fn cmp<F>(&mut self, args: &Vec<Arg>, cmp: F)
        where F: Fn(&i64, &i64) -> bool
    {
        let value = match cmp(&self.get_value(&args[0]), &self.get_value(&args[1])) {
            true => 1,
            false => 0
        };
        let cell = self.get_cell(&args[2]);
        *cell = value;
    }

    fn set_relative_offset(&mut self, args: &Vec<Arg>) {
        let value = self.get_value(&args[0]);
        self.relative_offset += value;
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}

fn part1(program: &Vec<i64>) -> i32 {
    let (sender, receiver) = channel();
    let mut robot = Intcode::new(program.clone(), sender, false);
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

fn part2(mut program: Vec<i64>) -> i64 {
    let (sender, receiver) = channel();
    program[0] = 2;

    let mut robot = Intcode::new(program, sender, true);
    thread::spawn(move || robot.exec().unwrap()).join().unwrap();
    let received: Vec<_> = receiver.iter().collect();
    *received.split_last().unwrap().0
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
    println!("{}", part1(&program));
    println!("{}", part2(program));
    Ok(())
}
