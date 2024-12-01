use std::fs;

mod day1;

fn main() {
    let data = fs::read_to_string("inputs/day1.txt").expect("Unable to read file");
    println!("Part 1 : {}", day1::part1(&data));
    println!("Part 2 : {}", day1::part2(&data));
}
