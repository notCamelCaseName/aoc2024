use std::fs;

mod day8;

fn main() {
    let data = fs::read_to_string("inputs/day8.txt").expect("Unable to read file");
    println!("Part 1 : {}", day8::part1(&data));
    println!("Part 2 : {}", day8::part2(&data));
}
