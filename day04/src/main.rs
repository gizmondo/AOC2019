use itertools::Itertools;

struct Properties {
    increasing: bool,
    has_group: bool,
    has_group_of_two: bool
}

fn digits(mut i: i32) -> impl Iterator<Item = i32>  {
    std::iter::from_fn(move || {
        if i == 0 {
            None
        } else {
            let next = i % 10;
            i = i / 10;
            Some(next)
        }
    })
}

fn solve(low: i32, high: i32) -> (i32, i32) {
    let mut result = (0, 0);
    for candidate in low..=high {
        let mut props = Properties {increasing: true, has_group: false, has_group_of_two: false};
        let mut prev_key = 10;
        for (key, group) in &digits(candidate).into_iter().group_by(|&d| d) {
            let group_size = group.fold(0, |acc, _| acc + 1);
            if key > prev_key {
                props.increasing = false;
                break;
            }
            if group_size == 2 {
                props.has_group = true;
                props.has_group_of_two = true;
            } else if group_size > 2 {
                props.has_group = true;
            }
            prev_key = key;
        }

        if props.increasing {
            if props.has_group {
                result.0 += 1;
            }
            if props.has_group_of_two {
                result.1 += 1;
            }
        }

    }
    result
}

fn main() {
    println!("{:?}", solve(171309, 643603));
}
