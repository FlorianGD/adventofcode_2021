use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

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

fn identify_digits(digits: &Input) -> [HashSet<char>; 10] {
    let mut number_to_pattern: [HashSet<char>; 10] = Default::default();
    // 6, 9 and 0 have 6 segments
    let mut sixes: Vec<HashSet<char>> = Vec::new();
    // 2, 3 and 5 have 5 segments
    let mut fives: Vec<HashSet<char>> = Vec::new();

    for d in digits {
        let len = d.len();
        if len == 7 {
            number_to_pattern[8] = d.chars().collect();
        } else if len == 4 {
            number_to_pattern[4] = d.chars().collect();
        } else if len == 3 {
            number_to_pattern[7] = d.chars().collect();
        } else if len == 2 {
            number_to_pattern[1] = d.chars().collect();
        } else if len == 6 {
            sixes.push(d.chars().collect());
        } else if len == 5 {
            fives.push(d.chars().collect());
        }
    }
    // sixes : 6, 9 and 0
    // 6 is the only in sixes which does not contain the same segments as 1
    number_to_pattern[6] = sixes
        .iter()
        .filter(|&x| x.intersection(&number_to_pattern[1]).count() == 1)
        .next()
        .unwrap()
        .to_owned();
    // 9 is the one which has 2 digits difference with 4
    number_to_pattern[9] = sixes
        .iter()
        .filter(|&x| x.difference(&number_to_pattern[4]).count() == 2)
        .next()
        .unwrap()
        .to_owned();
    // 0 is the remaining one
    number_to_pattern[0] = sixes
        .iter()
        .filter(|&x| x != &number_to_pattern[6] && x != &number_to_pattern[9])
        .next()
        .unwrap()
        .to_owned();
    // fives
    // 3 is the only one in fives which contains the same segments as 1
    number_to_pattern[3] = fives
        .iter()
        .filter(|&x| x.intersection(&number_to_pattern[1]).count() == 2)
        .next()
        .unwrap()
        .to_owned();
    // 2 is the one not in 9
    number_to_pattern[2] = fives
        .iter()
        .filter(|&x| x.difference(&number_to_pattern[9]).count() > 0)
        .next()
        .unwrap()
        .to_owned();
    // 5 is the one remaining
    number_to_pattern[5] = fives
        .iter()
        .filter(|&x| x != &number_to_pattern[2] && x != &number_to_pattern[3])
        .next()
        .unwrap()
        .to_owned();
    number_to_pattern
}

fn decode_digits(patterns: &[HashSet<char>; 10], output: &Output) -> usize {
    output
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| {
            let set: HashSet<char> = x.chars().collect();
            let digit = patterns.iter().position(|x| x == &set).unwrap();
            digit * 10_usize.pow(i.try_into().unwrap())
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(matches: &[(Input, Output)]) -> usize {
    matches
        .iter()
        .map(|(input, output)| decode_digits(&identify_digits(input), output))
        .sum()
}
