use std::error::Error;
use std::fs;


type AocResult<T> = std::result::Result<T, Box<Error>>;


macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}


fn exec(mut program: Vec<i32>) -> AocResult<i32> {
    for pc in (0..program.len()).step_by(4) {
        let opcode = program[pc];
        match opcode {
            1 | 2 => {
                let left_addr= program[pc + 1] as usize;
                let right_addr = program[pc + 2] as usize;
                let result_addr = program[pc + 3] as usize;

                let op = match opcode {
                    1 => i32::wrapping_add,
                    2 => i32::wrapping_mul,
                    _ => unreachable!()
                };
                program[result_addr] = op(program[left_addr], program[right_addr]);

            },
            99 => break,
            _ => return err!("Illegal opcode '{0}'", opcode)
        }
    }

    Ok(program[0])
}

fn part1(program: &Vec<i32>) -> AocResult<i32> {
    let mut parametrized = program.clone();
    parametrized[1] = 12;
    parametrized[2] = 2;
    exec(parametrized)
}

fn part2(program: &Vec<i32>) -> AocResult<i32> {
    let target = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut parametrized = program.clone();
            parametrized[1] = noun;
            parametrized[2] = verb;
            let answer = exec(parametrized)?;
            if answer == target {
                return Ok(100 * noun + verb);
            }
        }
    }
    err!("Could not find the target")
}


fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let program = input.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
    println!("{}", part1(&program)?);
    println!("{}", part2(&program)?);
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