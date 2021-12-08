use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Input = [String; 10];
type Output = [String; 4];

fn extract_digits(line: &str) -> (Input, Output) {
    let (input, output) = line.split_once(" | ").unwrap();
    lazy_static! {
        static ref DIGITS: Regex = Regex::new(r"([a-g]+)").unwrap();
    }
    let mut vec_input: Vec<_> = Vec::new();
    let mut vec_output: Vec<_> = Vec::new();
    for capture in DIGITS.captures_iter(input) {
        vec_input.push(capture[1].to_owned());
    }
    for capture in DIGITS.captures_iter(output) {
        vec_output.push(capture[1].to_owned());
    }
    let input: Input = vec_input.try_into().unwrap();
    let output: Output = vec_output.try_into().unwrap();
    (input, output)
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<(Input, Output)> {
    input.lines().map(|line| extract_digits(line)).collect()
}

#[aoc(day8, part1)]
pub fn part1(matches: &[(Input, Output)]) -> u32 {
    let mut count = 0;
    for (_, output) in matches {
        for group in output {
            match group.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn identify_digits(digits: &Input) -> HashMap<String, u8> {
    let mut known: HashMap<String, u8> = HashMap::new();

    // 6, 9 and 0 have 6 segments
    let mut sixes: Vec<String> = Vec::new();
    // 2, 3 and 5 have 5 segments
    let mut fives: Vec<String> = Vec::new();

    for d in digits {
        let len = d.len();
        if len == 7 {
            known.insert(d.chars().sorted().collect(), 8);
        } else if len == 4 {
            known.insert(d.chars().sorted().collect(), 4);
        } else if len == 3 {
            known.insert(d.chars().sorted().collect(), 7);
        } else if len == 2 {
            known.insert(d.chars().sorted().collect(), 1);
        } else if len == 6 {
            sixes.push(d.chars().sorted().collect());
        } else if len == 5 {
            fives.push(d.chars().sorted().collect());
        }
    }
    known
}

#[aoc(day8, part2)]
pub fn part2(matches: &[(Input, Output)]) -> u32 {
    0
}
