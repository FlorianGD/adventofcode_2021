#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    // let input = "16,1,2,0,4,2,7,1,2,14";
    let list = input.split(",").map(|x| x.parse().unwrap()).collect();
    list
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    let mut vec = input.clone();
    vec.sort();
    let median = vec[vec.len() / 2];
    vec.iter().map(|crab| (crab - median).abs()).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    let total: i32 = input.iter().sum();
    let len = input.len() as i32;
    let mean = total / len;
    input
        .iter()
        .map(|x| (x - mean).abs())
        .map(|x| (x * (x + 1) / 2))
        .sum()
}