use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[(Point, Point)]) -> usize {
    let mut map = HashMap::new();
    let horizontal_lines: Vec<&(Point, Point)> =
        input.iter().filter(|&(p1, p2)| p1.x == p2.x).collect();
    for (p1, p2) in horizontal_lines {
        let y_min = (p1.min(p2)).y;
        let y_max = (p1.max(p2)).y;
        let x = p1.x;
        for y in y_min..=y_max {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    let vertical_lines: Vec<&(Point, Point)> =
        input.iter().filter(|&(p1, p2)| p1.y == p2.y).collect();
    for (p1, p2) in vertical_lines {
        let x_min = (p1.min(p2)).x;
        let x_max = (p1.max(p2)).x;
        let y = p1.y;
        for x in x_min..=x_max {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }
    map.into_values().filter(|&v| v >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[(Point, Point)]) -> usize {
    let mut map = HashMap::new();
    let horizontal_lines: Vec<&(Point, Point)> =
        input.iter().filter(|&(p1, p2)| p1.x == p2.x).collect();
    for (p1, p2) in horizontal_lines {
        let y_min = (p1.min(p2)).y;
        let y_max = (p1.max(p2)).y;
        let x = p1.x;
        for y in y_min..=y_max {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    let vertical_lines: Vec<&(Point, Point)> =
        input.iter().filter(|&(p1, p2)| p1.y == p2.y).collect();
    for (p1, p2) in vertical_lines {
        let x_min = (p1.min(p2)).x;
        let x_max = (p1.max(p2)).x;
        let y = p1.y;
        for x in x_min..=x_max {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    let diagonals: Vec<&(Point, Point)> = input
        .iter()
        .filter(|&(p1, p2)| p1.y != p2.y && p1.x != p2.x)
        .collect();
    for (p1, p2) in diagonals {
        let origin = p1.min(p2);
        let dest = p1.max(p2);
        let x_min = origin.x;
        let x_max = dest.x;
        let y_step = if origin.y < dest.y { 1 } else { -1 };
        let mut y = origin.y;
        for x in x_min..=x_max {
            *map.entry((x, y)).or_insert(0) += 1;
            y += y_step;
        }
    }
    map.into_values().filter(|&v| v >= 2).count()
}
