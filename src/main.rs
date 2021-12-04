use adventofcode_2021::day04;
use std::{fs, time::Instant};

fn main() {
    let input: String = fs::read_to_string("input/2021/day04.txt").unwrap();
    let mut now = Instant::now();
    let parsed = day04::parse_input(&input);
    let mut elapsed_time = now.elapsed();
    println!("\t⏱️ parsing took {} µs.", elapsed_time.as_micros());

    now = Instant::now();
    let part1 = day04::part1(&parsed);
    println!("Part 1: {:#?}", part1);
    elapsed_time = now.elapsed();
    println!("\t⏱️ part 1 took {} µs.", elapsed_time.as_micros());

    now = Instant::now();
    let part2 = day04::part2(&parsed);
    println!("Part 2: {:?}", part2);
    elapsed_time = now.elapsed();
    println!("\t⏱️ part 2 took {} µs.", elapsed_time.as_micros());
}
