// #[aoc(day01)]
// pub fn generator(input: &str) -> Vec<u32> {
//     input.lines().map(|x| x.parse().unwrap()).collect()
// }
// 
// #[aoc(day01, part1)]
pub fn part1(input: Vec<u32>)-> u32 {
    let mut total = 0;
    let i1 = &input.clone()[..];
    let i2 = &input.clone()[1..];
    for (x, y) in i1.iter().zip(i2.iter()) {
        if x < y {
            total += 1;
        }
    }
    println!("{}", total);
    total
}

pub fn part2(input: Vec<u32>) -> u32 {
    let mut strides: Vec<u32> = Vec::new();
    for i in 0..input.len()-2 {
        strides.push(input[i..=i+2].iter().sum());
    }
    part1(strides)
}
