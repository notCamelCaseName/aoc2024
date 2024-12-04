use std::fs;

mod day4;

fn main() {
    let data = fs::read_to_string("inputs/day4.txt").expect("Unable to read file");
    println!("Part 1 : {}", day4::part1(&data));
    println!("Part 2 : {}", day4::part2(&data));
}
