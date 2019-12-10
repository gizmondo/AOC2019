use std::error::Error;
use std::fs;
use std::io::{BufReader, BufRead, Read, BufWriter, Write};

type AocResult<T> = std::result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

struct Intcode {
    program: Vec<i32>,
    pc: usize,
    input: BufReader<Box<dyn Read>>,
    output: BufWriter<Box<dyn Write>>,
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
    fn new(program: Vec<i32>, read: Box<Read>, write: Box<Write>) -> Intcode {
        Intcode {
            program,
            pc: 0,
            input: BufReader::new(read),
            output: BufWriter::new(write),
            halted: false
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
                Op::Jit => self.jump_if_true(&instruction.args),
                Op::Jif => self.jump_if_false(&instruction.args),
                _ => {
                    self.pc += len + 1;
                    match instruction.op {
                        Op::Add => self.add(&instruction.args),
                        Op::Mul => self.mul(&instruction.args),
                        Op::Input => self.input(&instruction.args)?,
                        Op::Output => self.output(&instruction.args)?,
                        Op::Lt => self.less(&instruction.args),
                        Op::Eq => self.equal(&instruction.args),
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
        let mut line = String::new();
        self.input.read_line(&mut line)?;
        self.program[args[0].value as usize] = line.trim().parse()?;
        Ok(())
    }

    fn output(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        self.output.write(format!("{}\n", self.get_value(&args[0])).as_bytes())?;
        Ok(())
    }

    fn jump_if_true(&mut self, args: &Vec<Arg>) {
        if self.get_value(&args[0]) != 0 {
            self.pc = self.get_value(&args[1]) as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jump_if_false(&mut self, args: &Vec<Arg>) {
        if self.get_value(&args[0]) == 0 {
            self.pc = self.get_value(&args[1]) as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less(&mut self, args: &Vec<Arg>) {
        let value;
        if self.get_value(&args[0]) < self.get_value(&args[1]) {
            value = 1;
        } else {
            value = 0;
        }
        self.program[args[2].value as usize] = value;
    }

    fn equal(&mut self, args: &Vec<Arg>) {
        let value;
        if self.get_value(&args[0]) == self.get_value(&args[1]) {
            value = 1;
        } else {
            value = 0;
        }
        self.program[args[2].value as usize] = value;
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
    Intcode::new(
        program,
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()),
    ).exec()
}