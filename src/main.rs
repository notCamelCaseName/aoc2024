use std::fs;

mod day2;

fn main() {
    let data = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");
    println!("Part 1 : {}", day2::part1(&data));
    println!("Part 2 : {}", day2::part2(&data));
}
