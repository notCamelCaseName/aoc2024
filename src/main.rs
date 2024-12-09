use std::fs;

mod day9;

fn main() {
    let data = fs::read_to_string("inputs/day9.txt").expect("Unable to read file");
    println!("Part 1 : {}", day9::part1(&data));
    println!("Part 2 : {}", day9::part2(&data));
}
