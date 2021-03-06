#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(String, u32)> {
    input
        .lines()
        .map(|x| {
            let mut y = x.split_whitespace();
            let action = y.next().unwrap();
            let direction: u32 = y.next().unwrap().parse().unwrap();
            (action.to_owned(), direction)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(String, u32)]) -> u32 {
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

#[aoc(day2, part2)]
pub fn part2(input: &[(String, u32)]) -> u32 {
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
