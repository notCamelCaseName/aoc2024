use std::fs;

mod day6;

fn main() {
    let data = fs::read_to_string("inputs/day6.txt").expect("Unable to read file");
    println!("Part 1 : {}", day6::part1(&data));
    println!("Part 2 : {}", day6::part2(&data));
}
