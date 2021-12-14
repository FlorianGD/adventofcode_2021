use std::collections::HashMap;
use itertools::Itertools;
type Pair = (char, char);
#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> (Vec<char>, HashMap<Pair, (Pair, Pair)>) {
    let (template, pairs) = input.split_once("\n\n").unwrap();
    let pairs = pairs
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(a,b)| (a.chars().next_tuple().unwrap(), b.chars().next().unwrap()))
        .map(|((a,b), c)| ((a,b), ((a,c), (c,b))))
        .collect();
    (template.chars().collect(), pairs)
}

fn step(counts: &HashMap<Pair, usize>, pairs: &HashMap<Pair, (Pair, Pair)>) -> HashMap<Pair, usize> {
    let mut new_counts = counts.clone();
    for (p, v) in counts {
        let (p1,p2) = pairs[&p];
        *new_counts.entry(p1).or_insert(0) += v;
        *new_counts.entry(p2).or_insert(0) += v;
        *new_counts.entry(*p).or_insert(0) -= v;
    }
    new_counts

}

fn score(counts: &HashMap<Pair, usize>, first: &char, last: &char) -> usize {
    let mut letters = HashMap::new();
    for ((a,b),v) in counts {
        *letters.entry(a).or_insert(0) +=v;
        *letters.entry(b).or_insert(0) +=v;
    }
    *letters.entry(first).or_insert(0) +=1;
    *letters.entry(last).or_insert(0)  +=1;
    println!("{:?}", letters);
    letters.values().max().unwrap() - letters.values().min().unwrap()
}

#[aoc(day14, part1)]
pub fn part1((template, pairs): &(Vec<char>, HashMap<Pair, (Pair, Pair)>)) -> usize {
    let mut counts = HashMap::new();
    for p in template.windows(2) {
        *counts.entry((p[0],p[1])).or_insert(0) +=1;
    }
    let mut new_counts = counts.clone();
    for _ in 0..40 {
        new_counts = step(&new_counts, pairs);
    }
    score(&new_counts, template.first().unwrap(), template.last().unwrap()) / 2
}

