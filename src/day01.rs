#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<u32>) -> u32 {
    let mut total = 0;
    for (x, y) in input.iter().zip(input[1..].iter()) {
        if x < y {
            total += 1;
        }
    }
    total
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<u32>) -> u32 {
    let mut strides: Vec<u32> = Vec::new();
    for i in 0..input.len() - 2 {
        strides.push(input[i..=i + 2].iter().sum());
    }
    part1(&strides)
}
