use std::fs;

mod day11;

fn main() {
    let data = fs::read_to_string("inputs/day11.txt").expect("Unable to read file");
    println!("Part 1 : {}", day11::part1(&data));
    println!("Part 2 : {}", day11::part2(&data));
}
