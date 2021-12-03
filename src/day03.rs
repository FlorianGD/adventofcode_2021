use std::{collections::HashMap, vec};

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

fn vec_to_int(vals: Vec<u8>) -> u32 {
    vals.iter().rev().enumerate().fold(0, |acc, (idx, val)| {
        acc + 2_u32.pow(idx.try_into().unwrap()) * (*val as u32)
    })
}

pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let positions = transpose(input.clone());
    let counts: Vec<_> = positions
        .iter()
        .map(|bits| {
            bits.iter().fold(HashMap::new(), |mut hm, bit| {
                *hm.entry(bit).or_insert(0) += 1;
                hm
            })
        })
        .collect();
    let maxs: Vec<u8> = counts
        .iter()
        .map(|hm| {
            hm.iter()
                .max_by(|&(_k1, v1), &(_k2, v2)| v1.cmp(&v2))
                .map(|(&k, _v)| *k as u8)
                .expect("huho")
        })
        .collect();

    let mins: Vec<u8> = maxs.iter().map(|x| 1 - x).collect();
    let maxs = vec_to_int(maxs);
    let mins = vec_to_int(mins);

    println!("{:?}", maxs);
    println!("{:?}", mins);
    maxs * mins
}

// pub fn part2(input: &Vec<(&str, u32)>) -> u32 {
//     let mut aim = 0;
//     let mut total = 0;
//     let mut depth = 0;
//     for &(x, y) in input {
//         match x {
//             "forward" => {
//                 total += y;
//                 depth += aim * y
//             }
//             "down" => aim += y,
//             _ => aim -= y,
//         }
//     }
//     total * depth
// }
