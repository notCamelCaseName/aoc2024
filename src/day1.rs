pub fn part1(data: &String) -> u32 {
    let (mut c1, mut c2): (Vec<i32>, Vec<i32>) = data.lines().map(|line| {
        let mut splitted_line = line.split_whitespace();
        (
            splitted_line.next().unwrap().parse::<i32>().unwrap(),
            splitted_line.next().unwrap().parse::<i32>().unwrap(),
        )
    }).unzip();
    c1.sort(); c2.sort();
    c1.iter().zip(c2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

pub fn part2(data: &String) -> i32 {
    let (mut c1, mut c2): (Vec<i32>, Vec<i32>) = data.lines().map(|line| {
        let mut splitted_line = line.split_whitespace();
        (
            splitted_line.next().unwrap().parse::<i32>().unwrap(),
            splitted_line.next().unwrap().parse::<i32>().unwrap(),
        )
    }).unzip();
    c1.iter().map(|n| n * c2.iter().filter(|a| *a == n).count() as i32).sum()
}
