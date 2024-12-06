use std::fs;

mod day5;

fn main() {
    let data = fs::read_to_string("inputs/day5.txt").expect("Unable to read file");
    println!("Part 1 : {}", day5::part1(&data));
    println!("Part 2 : {}", day5::part2(&data));
}
