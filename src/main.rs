use adventofcode_2021::day05;
use std::{fs, time::Instant};

fn main() {
    let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
    // let input: String = fs::read_to_string("input/2021/day04.txt").unwrap();
    // let mut now = Instant::now();
    let parsed = day05::parse_input(&input);
    // println!("{:?}", parsed);
    // let mut elapsed_time = now.elapsed();
    // println!("\t⏱️ parsing took {} µs.", elapsed_time.as_micros());

    // now = Instant::now();
    let part1 = day05::part1(&parsed);
    println!("Part 1: {:#?}", part1);
    // elapsed_time = now.elapsed();
    // println!("\t⏱️ part 1 took {} µs.", elapsed_time.as_micros());

    // now = Instant::now();
    // let part2 = day04::part2(&parsed);
    // println!("Part 2: {:?}", part2);
    // elapsed_time = now.elapsed();
    // println!("\t⏱️ part 2 took {} µs.", elapsed_time.as_micros());
}
