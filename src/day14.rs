use std::collections::HashMap;
use itertools::{interleave,Itertools};

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> (Vec<char>, HashMap<(char,char), char>) {
    let (template, pairs) = input.split_once("\n\n").unwrap();
    let pairs = pairs
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(a,b)| (a.chars().next_tuple().unwrap(), b.chars().next().unwrap()))
        .collect();
    (template.chars().collect(), pairs)
}

fn step(template: &Vec<char>, pairs: &HashMap<(char,char), char>) -> Vec<char> {
    let to_insert = Vec::new();
    for p in  template
        .clone()
        .iter()
        .tuple_windows() {
            to_insert.push(pairs[p]);
        }

    interleave(template
        ,to_insert)
        // .iter()
        .map(|a| a.to_owned())
        .collect()
}


#[aoc(day14, part1)]
pub fn part1((template, pairs): &(Vec<char>, HashMap<(char,char), char>)) -> u32 {
    println!("{:?}, {:?}", template, pairs);
    0
}

