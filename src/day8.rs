use gcd::Gcd;
use std::collections::{HashMap, HashSet};

fn parse(data: &String) -> HashMap<char, Vec<(usize, usize)>> {
    let mut res: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let mut new_value = if let Some(current_value) = res.get(&char) {
                current_value.clone()
            } else {
                Vec::new()
            };
            new_value.push((x, y));
            res.insert(char, new_value.clone());
        }
    }
    res
}

pub fn part1(data: &String) -> usize {
    let parsed_data = parse(data);
    let max_x = data.lines().next().unwrap().len();
    let max_y = data.lines().count();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for spots in parsed_data.values() {
        for i in 0..(spots.len() - 1) {
            for j in (i + 1)..spots.len() {
                let s1 = spots[i];
                let s2 = spots[j];
                let vec_s1_s2 = (s2.0 as isize - s1.0 as isize, s2.1 as isize - s1.1 as isize);
                let antinode1 = (s2.0 as isize + vec_s1_s2.0, s2.1 as isize + vec_s1_s2.1);
                let antinode2 = (s1.0 as isize - vec_s1_s2.0, s1.1 as isize - vec_s1_s2.1);
                if antinode1.0 >= 0
                    && antinode1.0 < max_x as isize
                    && antinode1.1 >= 0
                    && antinode1.1 < max_y as isize
                {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= 0
                    && antinode2.0 < max_x as isize
                    && antinode2.1 >= 0
                    && antinode2.1 < max_y as isize
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    antinodes.len()
}

pub fn part2(data: &String) -> usize {
    let parsed_data = parse(data);
    let max_x = data.lines().next().unwrap().len() as isize;
    let max_y = data.lines().count() as isize;
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for spots in parsed_data.values() {
        for i in 0..(spots.len() - 1) {
            for j in (i + 1)..spots.len() {
                let s1 = spots[i];
                let s2 = spots[j];
                let vec_s1_s2 = (s2.0 as isize - s1.0 as isize, s2.1 as isize - s1.1 as isize);
                let gcd = (vec_s1_s2.0.abs() as usize).gcd(vec_s1_s2.1.abs() as usize) as isize;
                let vec_s1_s2_norm = (vec_s1_s2.0 / gcd, vec_s1_s2.1 / gcd);
                for i in (-max_x - max_y)..(max_x + max_y) {
                    let antinode = (
                        s1.0 as isize + i * vec_s1_s2_norm.0,
                        s1.1 as isize + i * vec_s1_s2_norm.1,
                    );
                    if antinode.0 >= 0
                        && antinode.0 < max_x as isize
                        && antinode.1 >= 0
                        && antinode.1 < max_y as isize
                    {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }
    antinodes.len()
}
