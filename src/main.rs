use std::fs;

mod day3;

fn main() {
    let data = fs::read_to_string("inputs/day3.txt").expect("Unable to read file");
    println!("Part 1 : {}", day3::part1(&data));
    println!("Part 2 : {}", day3::part2(&data));
}
