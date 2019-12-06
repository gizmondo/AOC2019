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

    fn exec(&mut self) -> AocResult<i32> {
        while self.pc < self.program.len() && self.halted != true {
            let instruction = self.parse()?;
            let len = instruction.args.len();
            match instruction.op {
                Op::Add => self.add(&instruction.args),
                Op::Mul => self.mul(&instruction.args),
                Op::Input => self.input(&instruction.args)?,
                Op::Output => self.output(&instruction.args)?,
                Op::Halt => self.halt(),
            }
            self.pc += len + 1;
        }
        Ok(self.program[0])
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
        self.program[args[0].value as usize] = line.parse()?;
        Ok(())
    }

    fn output(&mut self, args: &Vec<Arg>) -> AocResult<()> {
        self.output.write(format!("{}\n", self.get_value(&args[0])).as_bytes())?;
        Ok(())
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}

fn exec(program: &Vec<i32>) -> AocResult<i32> {
    Intcode::new(
        program.clone(),
        Box::new("1".as_bytes()),
        Box::new(std::io::stdout()),
    ).exec()
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
    exec(&program)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() -> AocResult<()> {
        assert_eq!(exec(vec![1,0,0,0,99])?, 2);
        assert_eq!(exec(vec![2,3,0,3,99])?, 2);
        assert_eq!(exec(vec![1,1,1,4,99,5,6,0,99])?, 30);
        assert_eq!(exec(vec![2,4,4,5,99,0])?, 2);
        assert_eq!(exec(vec![1,9,10,3,2,3,11,0,99,30,40,50])?, 3500);
        assert!(exec(vec![666]).is_err());
        Ok(())
    }
}