use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> HashMap<u32, usize> {
    let list = input.split(',').map(|x| x.parse().unwrap());
    let mut state = HashMap::new();
    for elem in list {
        *state.entry(elem).or_insert(0) += 1
    }
    state
}

#[aoc(day6, part1)]
pub fn part1(input: &HashMap<u32, usize>) -> usize {
    let mut s = input.clone();
    for _ in 0..=79 {
        s = next_state(&s);
    }
    s.into_values().sum()
}

fn next_state(state: &HashMap<u32, usize>) -> HashMap<u32, usize> {
    let mut new_state = HashMap::new();
    for remaining in 0..=7 {
        new_state.insert(remaining, *state.get(&(remaining + 1)).unwrap_or(&0));
    }
    new_state.insert(8, *state.get(&0).unwrap_or(&0));
    *new_state.entry(6).or_insert(0) += new_state[&8];
    new_state
}

#[aoc(day6, part2)]
pub fn part2(input: &HashMap<u32, usize>) -> usize {
    let mut s = input.clone();
    for _ in 0..=255 {
        s = next_state(&s);
    }
    s.into_values().sum()
}
