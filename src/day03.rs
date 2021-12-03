use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| {
            x.chars().collect()
            // let mut y = x.split_whitespace();
            // let action = y.next().unwrap();
            // let direction: u32 = y.next().unwrap().parse().unwrap();
            // (action, direction)
        })
        .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn part1(input: &Vec<Vec<char>>) -> u32 {
    let positions = transpose(input.clone());
    let counts: Vec<_> = positions.iter().map(|bits| {
        bits.iter().fold(HashMap::new(), |mut hm, bit| {
           *hm.entry(bit).or_insert(0) += 1;
           hm
        })
    }).collect();
    let maxs: Vec<_> = counts.iter().map(|hm| {
        hm.iter()
            .max_by(|(_k1, v1), (_k2, v2)| v1.cmp(&v2))
            .map(|(k, _v)| k)
            .expect("huho")
    }).collect();

    println!("{:?}", maxs); 
    0
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
