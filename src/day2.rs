pub fn part1(data: &String) -> usize {
    data.lines()
        .filter(|line| {
            let parsed_line = line
                .split_whitespace()
                .map(|tok| tok.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut res = true;
            let mut last_diff = 0;
            for i in 1..parsed_line.len() {
                let diff = parsed_line[i] - parsed_line[i - 1];
                res &= diff * last_diff >= 0 && diff.abs() >= 1 && diff.abs() <= 3;
                last_diff = diff;
            }
            res
        })
        .count()
}

pub fn part2(data: &String) -> usize {
    // Dirty but it works
    data.lines()
        .filter(|line| {
            let parsed_line = line
                .split_whitespace()
                .map(|tok| tok.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut res = true;
            let mut last_diff = 0;
            for i in 1..parsed_line.len() {
                let diff = parsed_line[i] - parsed_line[i - 1];
                res &= diff * last_diff >= 0 && diff.abs() >= 1 && diff.abs() <= 3;
                last_diff = diff;
            }
            if res {
                return res;
            }
            for n in 0..parsed_line.len() {
                let mut new_parsed_line = parsed_line.clone();
                new_parsed_line.remove(n);
                res = true;
                last_diff = 0;
                for i in 1..new_parsed_line.len() {
                    let diff = new_parsed_line[i] - new_parsed_line[i - 1];
                    res &= diff * last_diff >= 0 && diff.abs() >= 1 && diff.abs() <= 3;
                    last_diff = diff;
                }
                if res {
                    return res;
                }
            }
            res
        })
        .count()
}
