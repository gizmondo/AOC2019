use std::fs;
use std::collections::HashMap;
use regex::Regex;

type AocResult<T> = std::result::Result<T, Box<std::error::Error>>;

fn part1(orbited_by: &HashMap<String, Vec<String>>) -> i32 {
    let mut result = 0;
    let mut stack = vec![("COM".to_owned(), 0)];
    while !stack.is_empty() {
        let (elem, counter) = stack.pop().unwrap();
        result += counter;
        if let Some(deps) = orbited_by.get(&elem) {
            stack.extend(deps.iter().map(|dep| (dep.to_owned(), counter + 1)));
        }
    }
    result
}

#[derive(PartialEq)]
enum Answer {
    Final(i32),
    Partial(i32),
}

fn search(key: &str, orbited_by: &HashMap<String, Vec<String>>) -> Option<Answer> {
    if key == "SAN" || key == "YOU" {
        return Some(Answer::Partial(0))
    }

    match orbited_by.get(key) {
        None => None,
        Some(deps) => {
            let sub_search: Vec<_> = deps.iter().map(|x| search(x, orbited_by)).filter_map(|x| x).collect();
            match sub_search.len() {
                0 => None,
                1 => {
                    match sub_search[0] {
                        Answer::Partial(val) => Some(Answer::Partial(val + 1)),
                        Answer::Final(val) => Some(Answer::Final(val))
                    }
                }
                2 => {
                    let mut res = 0;
                    for i in 0..=1 {
                        if let Answer::Partial(val) = sub_search[i] {
                            res += val
                        }
                    }
                    Some(Answer::Final(res))
                },
                _ => unreachable!()
            }
        }
    }
}

fn part2(orbited_by: &HashMap<String, Vec<String>>) -> AocResult<i32> {
    match search("COM", &orbited_by) {
        Some(Answer::Final(val)) => Ok(val),
        _ => Err(Box::from("Transfer not found"))
    }
}

fn main() -> AocResult<()> {
    let mut orbited_by: HashMap<String, Vec<String>> = HashMap::new();
    let input = fs::read_to_string("input.txt")?;
    let re = Regex::new(r"([a-zA-Z0-9]+)\)([a-zA-Z0-9]+)")?;
    for cap in re.captures_iter(&input) {
        let entry = orbited_by.entry(cap[1].to_owned()).or_insert(Vec::new());
        entry.push(cap[2].to_owned());
    }
    println!("{}", part1(&orbited_by));
    println!("{}", part2(&orbited_by)?);
    Ok(())
}
