use std::error::Error;
use std::fs;


type AocResult<T> = std::result::Result<T, Box<Error>>;


fn calc_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn part1(masses: &Vec<i32>) -> i32 {
    masses.into_iter().fold(0, |sum, &mass| sum + calc_fuel(mass))
}

fn part2(masses: &Vec<i32>) -> i32 {
    let mut total = 0;
    for &mass in masses {
        let mut fuel = calc_fuel(mass);
        while fuel > 0 {
            total += fuel;
            fuel = calc_fuel(fuel);
        }
    }
    total
}

fn main() -> AocResult<()> {
    let input = fs::read_to_string("input.txt")?;
    let masses = input.split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;

    println!("{0}", part1(&masses));
    println!("{0}", part2(&masses));

    Ok(())
}
