use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, channel};

type AocResult<T> = std::result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

struct Intcode {
    program: Vec<i64>,
    heap: HashMap<usize, i64>,
    pc: usize,
    relative_offset: i64,
    input: Receiver<i64>,
    output: Sender<i64>,
    halted: bool
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
    fn new(program: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> Intcode {
        Intcode {
            program, input, output,
            heap: HashMap::new(), pc: 0, relative_offset: 0, halted: false
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

    fn exec(&mut self) -> AocResult<()> {
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
            self.heap.entry(address as usize).or_insert(0)
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
        *cell = value0 + value1;
    }

    fn input(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        let value = self.input.recv()?;
        let cell = self.get_cell(&args[0]);
        *cell = value;
        Ok(())
    }

    fn output(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        let value = self.get_value(&args[0]);
        self.output.send(value)?;
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
        self.relative_offset = value;
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}

fn part1(program: &Vec<i64>) -> AocResult<i64> {
    let (sender, receiver) = channel();
    sender.send(1)?;
    let mut intcode = Intcode::new(program.clone(), receiver, sender);
    intcode.exec()?;
    Ok(intcode.input.recv()?)
}


fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part1(&program)?);
    Ok(())
}