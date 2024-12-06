use std::collections::HashMap;

fn parse_input(data: &String) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let order_rules = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            (
                line[0..2].parse::<i32>().unwrap(),
                line[3..].parse::<i32>().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut acc: HashMap<i32, Vec<i32>>, (a, b)| {
            if let Some(v) = acc.get_mut(&a) {
                v.push(b);
            } else {
                acc.insert(a, vec![b]);
            }
            acc
        });
    let print_sequences = data
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|tok| tok.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    (order_rules, print_sequences)
}

pub fn part1(data: &String) -> i32 {
    let (order_rules, print_sequences) = parse_input(data);
    print_sequences
        .iter()
        .filter(|sequence| {
            for i in 0..(sequence.len() - 1) {
                for j in (i + 1)..sequence.len() {
                    if let Some(rule) = order_rules.get(&sequence[j]) {
                        if rule.contains(&sequence[i]) {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .map(|sequence| sequence[sequence.len() / 2])
        .sum()
}

pub fn part2(data: &String) -> i32 {
    let (order_rules, print_sequences) = parse_input(data);
    print_sequences
        .iter()
        .filter(|sequence| {
            for i in 0..(sequence.len() - 1) {
                for j in (i + 1)..sequence.len() {
                    if let Some(rule) = order_rules.get(&sequence[j]) {
                        if rule.contains(&sequence[i]) {
                            return true;
                        }
                    }
                }
            }
            false
        })
        .map(|sequence| {
            let mut sequence = sequence.clone();
            loop {
                for i in 0..(sequence.len() - 1) {
                    for j in (i + 1)..sequence.len() {
                        if let Some(rule) = order_rules.get(&sequence[j]) {
                            if rule.contains(&sequence[i]) {
                                sequence.swap(i, j);
                            }
                        }
                    }
                }
                break;
            }
            sequence
        })
        .map(|sequence| sequence[sequence.len() / 2])
        .sum()
}
