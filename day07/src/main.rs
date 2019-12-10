use std::error::Error;
use std::fs;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

type AocResult<T> = std::result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

struct Intcode {
    program: Vec<i32>,
    pc: usize,
    input: Receiver<i32>,
    output: Sender<i32>,
    halted: bool
}

#[derive(Debug)]
enum Mode {
    Address,
    Immediate
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    Jit,
    Jif,
    Lt,
    Eq,
    Halt
}

#[derive(Debug)]
struct Arg {
    value: i32,
    mode: Mode
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    args: Vec<Arg>
}

impl Intcode {
    fn new(program: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> Intcode {
        Intcode { program, input, output, pc: 0, halted: false }
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
                illegal => return err!("Illegal mode '{}'", illegal)
            };
            args.push(Arg {value, mode});
            modes = modes / 10;
        }
        Ok(Instruction {op, args})
    }

    fn exec(&mut self) -> AocResult<()> {
        while self.pc < self.program.len() && self.halted != true {
            let instruction = self.parse()?;
            let len = instruction.args.len();
            match instruction.op {
                Op::Halt => self.halt(),
                Op::Jit => self.jump(&instruction.args, i32::ne),
                Op::Jif => self.jump(&instruction.args, i32::eq),
                _ => {
                    self.pc += len + 1;
                    match instruction.op {
                        Op::Add => self.add(&instruction.args),
                        Op::Mul => self.mul(&instruction.args),
                        Op::Input => self.input(&instruction.args)?,
                        Op::Output => self.output(&instruction.args)?,
                        Op::Lt => self.cmp(&instruction.args, i32::lt),
                        Op::Eq => self.cmp(&instruction.args, i32::eq),
                        _ => unreachable!()
                    }
                }
            }
        }
        Ok(())
    }

    fn get_value(&self, arg: &Arg) -> i32 {
        match arg.mode {
            Mode::Immediate => arg.value,
            Mode::Address => self.program[arg.value as usize]
        }
    }

    fn add(&mut self, args: &Vec<Arg>) {
        self.program[args[2].value as usize] = self.get_value(&args[0]) + self.get_value(&args[1]);
    }

    fn mul(&mut self, args: &Vec<Arg>) {
        self.program[args[2].value as usize] = self.get_value(&args[0]) * self.get_value(&args[1]);
    }

    fn input(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        self.program[args[0].value as usize] = self.input.recv()?;
//        println!("Input: {}", self.program[args[0].value as usize]);
        Ok(())
    }

    fn output(&mut self, args: &Vec<Arg>) -> AocResult<()> {
//        println!("Output: {}", self.get_value(&args[0]));
        self.output.send(self.get_value(&args[0]))?;
        Ok(())
    }

    fn jump<F>(&mut self, args: &Vec<Arg>, cond: F)
        where F: Fn(&i32, &i32) -> bool
    {
        if cond(&self.get_value(&args[0]), &0) {
            self.pc = self.get_value(&args[1]) as usize;
        } else {
            self.pc += args.len() + 1;
        }
    }

    fn cmp<F>(&mut self, args: &Vec<Arg>, cmp: F)
        where F: Fn(&i32, &i32) -> bool
    {
        let value = match cmp(&self.get_value(&args[0]), &self.get_value(&args[1])) {
            true => 1,
            false => 0
        };
        self.program[args[2].value as usize] = value;
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}

fn permutations(seq: &Vec<i32>) -> Vec<Vec<i32>> {
    if seq.len() == 1 {
        return vec![seq.clone()]
    }
    let mut result = Vec::new();
    for i in 0..seq.len() {
        let mut base = seq.clone();
        let elem = base.remove(i);
        for mut perm in permutations(&base) {
            perm.push(elem);
            result.push(perm);
        }
    }
    result
}

fn part1(program: &Vec<i32>) -> AocResult<i32> {
    let mut result = std::i32::MIN;
    for perm in permutations(&(0..5).collect()) {
        let (sender, mut receiver) = channel();
        sender.send(perm[0])?;
        sender.send(0)?;

        for i in 0..5 {
            let (new_sender, new_receiver) = channel();
            if i != 4 {
                new_sender.send(perm[i + 1])?;
            }
            let mut intcode = Intcode::new(program.clone(), receiver, new_sender);
            receiver = new_receiver;
            intcode.exec()?;
        }

        let output = receiver.recv()?;
        if output > result {
            result = output;
        }
    }
    Ok(result)
}

fn part2(program: &Vec<i32>) -> AocResult<i32> {
    let mut result = std::i32::MIN;
    for perm in permutations(&(5..10).collect()) {
        let mut senders = vec![];
        let mut receivers = vec![];
        for phase in perm {
            let (sender, receiver) = channel();
            sender.send(phase)?;
            senders.push(sender);
            receivers.push(receiver);
        }
        receivers.rotate_right(1);
        senders[4].send(0)?;

        let mut intcodes: Vec<Intcode> = receivers
            .into_iter()
            .zip(senders)
            .map(|(r, s)| Intcode::new(program.clone(), r, s))
            .collect();
        intcodes.rotate_left(1);

        for (i, mut intcode) in intcodes.into_iter().enumerate() {
            if i != 4 {
                thread::spawn(move || intcode.exec().unwrap());
            } else {
                intcode.exec()?;
                result = std::cmp::max(result, intcode.input.recv()?);
            }
        }
    }
    Ok(result)
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part1(&program)?);
    println!("{:?}", part2(&program)?);
    Ok(())
}