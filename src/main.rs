use adventofcode_2021::day01;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input/2021/day01.txt").unwrap();
    let parsed: Vec<u32> = input
        .lines()
        .map(|l| {
            l.parse().unwrap()
        })
    .collect();
    day01::part1(parsed.clone());
    day01::part2(parsed);
}
