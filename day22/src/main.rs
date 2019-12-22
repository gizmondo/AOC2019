use std::fs;
use std::error::Error;
use std::result::Result;
use regex::Regex;

type AocResult<T> = Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

enum Action {
    NewStack,
    Increment(usize),
    Cut(i32),
}

fn part1(shuffle: &Vec<Action>) -> usize {
    const SIZE: usize = 10007;
    let mut stack: Vec<_> = (0..SIZE as i32).collect();
    for action in shuffle.into_iter() {
        match action {
            Action::NewStack => stack.reverse(),
            Action::Cut(val) => if val > &0 {
                stack.rotate_left(*val as usize);
            } else {
                stack.rotate_right(-val as usize);
            },
            Action::Increment(shift) => {
                let mut new_stack = vec![0; SIZE];
                for (i, val) in stack.into_iter().enumerate() {
                    new_stack[(i * shift) % SIZE] = val
                }
                stack = new_stack;
            }
        }
    }
    stack.into_iter().position(|x| x == 2019).unwrap()
}

fn main() -> AocResult<()> {
    let mut shuffle = vec![];
    let input = fs::read_to_string("input.txt")?;
    let re = Regex::new(r"(deal into new stack)|(deal with increment ([0-9]+))|(cut (\-?[0-9]+))")?;
    for cap in re.captures_iter(&input) {
        let action;
        if let Some(_) = cap.get(1) {
            action = Action::NewStack;
        } else if let Some(_) = cap.get(2) {
            action = Action::Increment(cap[3].parse()?);
        } else if let Some(_) = cap.get(4) {
            action = Action::Cut(cap[5].parse()?);
        } else {
            return err!("Unknown action");
        }
        shuffle.push(action);
    }

    println!("{}", part1(&shuffle));
    Ok(())
}
