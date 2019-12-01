use std::error::Error;
use std::fs;


type AocResult<T> = std::result::Result<T, Box<Error>>;


fn part1(masses: &Vec<i32>) -> AocResult<i32> {
    let mut total = 0;
    for mass in masses {
        total += (mass / 3) - 2;
    }
    Ok(total)
}

fn part2(masses: &Vec<i32>) -> AocResult<i32> {
    let mut total = 0;
    for mass in masses {
        let mut fuel = (mass / 3) - 2;
        while fuel > 0 {
            total += fuel;
            fuel = (fuel / 3) - 2;
        }
    }
    Ok(total)
}


fn main() -> AocResult<()> {
    let input: String = fs::read_to_string("input.txt")?;
    let masses = input.split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;

    println!("{0}", part1(&masses)?);
    println!("{0}", part2(&masses)?);

    Ok(())
}
