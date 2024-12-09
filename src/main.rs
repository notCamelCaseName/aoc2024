use std::fs;

mod day7;

fn main() {
    let data = fs::read_to_string("inputs/day7.txt").expect("Unable to read file");
    println!("Part 1 : {}", day7::part1(&data));
    println!("Part 2 : {}", day7::part2(&data));
}
