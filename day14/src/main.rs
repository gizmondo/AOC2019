use std::fs;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type AocResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Reaction {
    input: Vec<(f64, String)>,
    output: f64
}

fn parse_chemical(s: &str) -> AocResult<(f64, String)> {
    let sides: Vec<_> = s.trim().split_whitespace().collect();
    Ok((sides[0].parse::<f64>()?, sides[1].to_owned()))
}

fn kahn(reactions: &HashMap<String, Reaction>) -> Vec<&str> {
    let mut result = Vec::new();

    let mut in_degrees = HashMap::new();
    for reaction in reactions.values() {
        for (_, s) in reaction.input.iter() {
            *in_degrees.entry(s).or_insert(0) += 1;
        }
    }

    let mut stack = vec!["FUEL"];
    while let Some(key) = stack.pop() {
        result.push(key);
        if let Some(reaction) = reactions.get(key) {
            for (_, s) in reaction.input.iter() {
                let in_degree = in_degrees.get_mut(&s).unwrap();
                *in_degree -= 1;
                if *in_degree == 0 {
                    stack.push(&s);
                }
            }
        }
    }
    assert_eq!(result.len(), reactions.len() + 1);
    result.pop();
    result
}

fn calc_required_ore(reactions: &HashMap<String, Reaction>, order: &Vec<&str>, fuel: &i64) -> i64 {
    let mut quantities = HashMap::new();
    quantities.insert("FUEL", *fuel as f64);
    for &req in order {
        let quantity = quantities.remove(req).unwrap();
        let reaction = reactions.get(req).unwrap();
        let mult = (quantity / reaction.output as f64).ceil();
        for (q, chem) in reaction.input.iter() {
            *quantities.entry(&chem).or_insert(0.0) += q * mult
        }
    }
    quantities.get("ORE").unwrap().clone() as i64
}

fn part1(reactions: &HashMap<String, Reaction>) -> i64 {
    let order = kahn(reactions);
    calc_required_ore(reactions, &order, &1)
}

fn part2(reactions: &HashMap<String, Reaction>) -> i64 {
    const ORE: i64 = 1_000_000_000_000;
    let order = kahn(reactions);
    let mut low = 1;
    let mut high = 1_000_000_000;
    assert!(calc_required_ore(reactions, &order, &low) <= ORE);
    assert!(calc_required_ore(reactions, &order, &high) > ORE);
    while low < high - 1 {
        let middle = (low + high) / 2;
        if calc_required_ore(reactions, &order, &middle) > ORE {
            high = middle;
        } else {
            low = middle
        }
    }
    low
}

fn main() -> AocResult<()> {
    let reader = BufReader::new(fs::File::open("input.txt")?);

    let mut reactions = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let sides: Vec<_> = line.split("=>").collect();
        let input = sides[0].split(",").map(parse_chemical).collect::<AocResult<Vec<_>>>()?;
        let (output, chemical) = parse_chemical(sides[1])?;
        reactions.insert(chemical, Reaction { output, input });
    }

    println!("{}", part1(&reactions));
    println!("{}", part2(&reactions));
    Ok(())
}
