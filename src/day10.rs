use std::collections::HashSet;

fn parse(data: &String) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(36).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_possible_neighbors(data: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::with_capacity(4);
    let current_height = data[y][x];
    if x > 0 {
        if data[y][x - 1] == current_height + 1 {
            res.push((x - 1, y));
        }
    }
    if y > 0 {
        if data[y - 1][x] == current_height + 1 {
            res.push((x, y - 1));
        }
    }
    if x < data[0].len() - 1 {
        if data[y][x + 1] == current_height + 1 {
            res.push((x + 1, y));
        }
    }
    if y < data.len() - 1 {
        if data[y + 1][x] == current_height + 1 {
            res.push((x, y + 1));
        }
    }
    res
}

fn explore_part1(
    data: &Vec<Vec<u32>>,
    (x, y): (usize, usize),
    seen_positions: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut res = 0;
    seen_positions.insert((x, y));
    if data[y][x] == 9 {
        return 1;
    }
    for neighbor in get_possible_neighbors(data, (x, y)) {
        if !seen_positions.contains(&neighbor) {
            res += explore_part1(data, neighbor, seen_positions)
        }
    }
    res
}

fn explore_part2(data: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> usize {
    let mut res = 0;
    if data[y][x] == 9 {
        return 1;
    }
    for neighbor in get_possible_neighbors(data, (x, y)) {
        res += explore_part2(data, neighbor)
    }
    res
}

pub fn part1(data: &String) -> usize {
    let parsed_data = parse(data);
    parsed_data
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, v)| {
                    if *v == 0 {
                        explore_part1(&parsed_data, (x, y), &mut HashSet::new())
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(data: &String) -> usize {
    let parsed_data = parse(data);
    parsed_data
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, v)| {
                    if *v == 0 {
                        explore_part2(&parsed_data, (x, y))
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
