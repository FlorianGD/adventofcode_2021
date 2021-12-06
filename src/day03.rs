use std::collections::HashMap;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect()
}

// from stackoverflow
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn vec_to_int(vals: Vec<u32>) -> u32 {
    vals.iter().rev().enumerate().fold(0, |acc, (idx, val)| {
        acc + 2_u32.pow(idx.try_into().unwrap()) * val
    })
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let positions = transpose(input.clone());
    let counts = get_counts(&positions);
    let maxs = get_maxs(&counts);

    let mins: Vec<u32> = maxs.iter().map(|x| 1 - x).collect();
    let maxs = vec_to_int(maxs);
    let mins = vec_to_int(mins);

    maxs * mins
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut new_input = input.clone();
    let mut positions;
    let mut idx = 0;
    while new_input.len() > 1 {
        positions = transpose(new_input.clone());
        let counts = get_counts(&positions);
        let max = get_maxs(&counts[idx..counts.len()])[0];
        new_input = new_input.into_iter().filter(|x| x[idx] == max).collect();
        // println!("input: {:?}", &new_input);
        // println!("posit: {:?}", &positions);
        idx += 1;
    }
    let max = vec_to_int(new_input[0].clone());

    new_input = input.clone();
    idx = 0;
    while new_input.len() > 1 {
        positions = transpose(new_input.clone());
        let counts = get_counts(&positions);
        let min = get_mins(&counts[idx..counts.len()])[0];
        new_input = new_input.into_iter().filter(|x| x[idx] == min).collect();
        idx += 1;
    }
    let min = vec_to_int(new_input[0].clone());
    max * min
}

fn get_maxs(counts: &[HashMap<&u32, i32>]) -> Vec<u32> {
    let maxs: Vec<u32> = counts
        .iter()
        .map(
            |hm| match hm.get(&0).unwrap_or(&0) > hm.get(&1).unwrap_or(&0) {
                true => 0,
                false => 1,
            },
        )
        .collect();
    maxs
}
fn get_mins(counts: &[HashMap<&u32, i32>]) -> Vec<u32> {
    let maxs: Vec<u32> = counts
        .iter()
        .map(
            |hm| match hm.get(&0).unwrap_or(&0) <= hm.get(&1).unwrap_or(&0) {
                true => 0,
                false => 1,
            },
        )
        .collect();
    maxs
}
fn get_counts(positions: &[Vec<u32>]) -> Vec<HashMap<&u32, i32>> {
    let counts: Vec<_> = positions
        .iter()
        .map(|bits| {
            bits.iter().fold(HashMap::new(), |mut hm, bit| {
                *hm.entry(bit).or_insert(0) += 1;
                hm
            })
        })
        .collect();
    counts
}
