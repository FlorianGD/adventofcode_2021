use adventofcode_2021::day03;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input/2021/day03.txt").unwrap();
    //     let input = "\
    // 00100
    // 11110
    // 10110
    // 10111
    // 10101
    // 01111
    // 00111
    // 11100
    // 10000
    // 11001
    // 00010
    // 01010";
    let parsed = day03::parse_input(&input);
    let part1 = day03::part1(&parsed);
    //let part2 = day03::part2(&parsed);
    println!("Part 1: {}", part1);
    //println!("Part 2: {}", part2);
}
