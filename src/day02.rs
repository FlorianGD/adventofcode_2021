pub fn parse_input(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|x| {
            let mut y = x.split_whitespace();
            let action = y.next().unwrap();
            let direction: u32 = y.next().unwrap().parse().unwrap();
            (action, direction)
        })
        .collect()
}

pub fn part1(input: &Vec<(&str, u32)>) -> u32 {
    let mut total = 0;
    let mut depth = 0;
    for (x, y) in input {
        match x.as_ref() {
            "forward" => total += y,
            "down" => depth += y,
            _ => depth -= y,
        }
    }
    total * depth
}

pub fn part2(input: &Vec<(&str, u32)>) -> u32 {
    let mut aim = 0;
    let mut total = 0;
    let mut depth = 0;
    for (x, y) in input {
        match x.as_ref() {
            "forward" => {
                total += y;
                depth += aim * y
            }
            "down" => aim += y,
            _ => aim -= y,
        }
    }
    total * depth
}
