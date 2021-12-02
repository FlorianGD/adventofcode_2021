pub fn parse_input(input: &str) -> Vec<(String,u32)> {
    input.lines().map(|x| {
        let mut y = x.split_whitespace();
        let action = y.next().unwrap().to_string();
        let direction: u32 = y.next().unwrap().parse().unwrap();
        (action, direction)
    }).collect()
}

pub fn part2(input: &Vec<(String, u32)>) -> u32 {
    let mut aim = 0;
    let mut total = 0;
    let mut depth =0;
    for (x, y) in input.iter() {
        if x == "forward" {
            total += y;
            depth += aim * y
        } else if x == "down" {
            aim += y
        } else {
            aim -= y
        }
    }
    total * depth 
}

pub fn part1(input: &Vec<(String, u32)>) -> u32 {
    let mut total = 0;
    let mut depth =0;
    for (x, y) in input.iter() {
        if x == "forward" {
            total += y
        } else if x == "down" {
            depth += y
        } else {
            depth -= y
        }
    }
    total * depth
}

