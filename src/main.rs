use adventofcode_2021::day02;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input/2021/day02.txt").unwrap();
    let parsed = day02::parse_input(&input);
    let part1 = day02::part1(&parsed);
    let part2 = day02::part2(&parsed);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}