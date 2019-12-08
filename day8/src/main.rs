use std::collections::HashMap;

type AocResult<T> = std::result::Result<T, Box<std::error::Error>>;

const W: usize = 25;
const H: usize = 6;

fn counter<I, T>(it: I) -> HashMap<T, i32>
where
    I: Iterator<Item=T>,
    T: std::cmp::Eq + std::hash::Hash
{
    let mut result = HashMap::new();
    for item in it {
        let count = result.entry(item).or_insert(0);
        *count += 1;
    }
    result
}

fn part1(image: &Vec<i32>) -> i32 {
    image
        .chunks(W * H)
        .map(|layer| {
            let c = counter(layer.into_iter());
            (*c.get(&0).unwrap_or(&0), *c.get(&1).unwrap_or(&0) * *c.get(&2).unwrap_or(&0))
        })
        .min()
        .unwrap_or((-1, -1))
        .1
}

fn part2(image: &Vec<i32>) {
    let mut merged = vec![2; W * H];
    for layer in image.chunks(W * H) {
        for (i, &color) in layer.into_iter().enumerate() {
            if merged[i] == 2 && color != 2 {
                merged[i] = color;
            }
        }
    }
    for row in merged.chunks(W) {
        let s: String = row
            .into_iter()
            .map(|i| match i {
                0 => ' ',
                1 => 'O',
                _ => panic!("Illegal colour")
            })
            .collect();
        println!("{}", s);
    }
}


fn main() -> AocResult<()> {
    let input = std::fs::read_to_string("input.txt")?
        .trim()
        .chars()
        .map(|ch| ch as i32 - '0' as i32)
        .collect();
    println!("{}", part1(&input));
    part2(&input);
    Ok(())
}
