use std::fs;

mod day10;

fn main() {
    let data = fs::read_to_string("inputs/day10.txt").expect("Unable to read file");
    println!("Part 1 : {}", day10::part1(&data));
    println!("Part 2 : {}", day10::part2(&data));
}
