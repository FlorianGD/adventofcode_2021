use adventofcode_2021::day01;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input/2021/day01.txt").unwrap();
    let parsed = day01::parse_input(&input);
    let part1 = day01::part1(&parsed);
    let part2 = day01::part2(&parsed);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
