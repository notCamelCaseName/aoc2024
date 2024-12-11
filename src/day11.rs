use std::collections::HashMap;

fn parse(data: &String) -> Vec<String> {
    data.trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn remove_trailing_zeros(s: &str) -> String {
    s.parse::<usize>().unwrap().to_string()
}

fn compute_rec(
    stone: String,
    iterations: usize,
    pre_computed: &mut HashMap<(String, usize), usize>,
) -> usize {
    let res = if iterations == 0 {
        1
    } else if pre_computed.contains_key(&(stone.clone(), iterations)) {
        pre_computed[&(stone.clone(), iterations)]
    } else if stone == "0" {
        compute_rec(
            "1".to_string(),
            iterations - 1,
            pre_computed,
        )
    } else if stone.len() % 2 == 0 {
        let split = stone.len() / 2;
        compute_rec(
            remove_trailing_zeros(&stone[0..split]),
            iterations - 1,
            pre_computed,
        ) + compute_rec(
            remove_trailing_zeros(&stone[split..stone.len()]),
            iterations - 1,
            pre_computed,
        )
    } else {
        compute_rec(
            (stone.parse::<usize>().unwrap() * 2024).to_string(),
            iterations - 1,
            pre_computed,
        )
    };
    pre_computed.insert((stone, iterations), res);
    res
}

pub fn part1(data: &String) -> usize {
    let parsed_data = parse(data);
    let mut default_known = HashMap::new();
    default_known.insert(("0".to_string(), 1_usize), 1);
    parsed_data
        .iter()
        .map(|stone| compute_rec(stone.clone(), 25, &mut default_known))
        .sum()
}

pub fn part2(data: &String) -> usize {
    let parsed_data = parse(data);
    let mut default_known = HashMap::new();
    default_known.insert(("0".to_string(), 1_usize), 1);
    parsed_data
        .iter()
        .map(|stone| compute_rec(stone.clone(), 75, &mut default_known))
        .sum()
}
