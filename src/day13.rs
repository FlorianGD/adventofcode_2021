use std::{collections::HashSet, str::FromStr};

type Coord = (u32, u32);

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
}
impl FromStr for Axis {
    type Err = ();

    fn from_str(input: &str) -> Result<Axis, Self::Err> {
        match input.chars().last() {
            Some('x') => Ok(Axis::X),
            Some('y') => Ok(Axis::Y),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (HashSet<Coord>, Vec<(Axis, u32)>) {
    //     let input = "\
    // 6,10
    // 0,14
    // 9,10
    // 0,3
    // 10,4
    // 4,11
    // 6,0
    // 6,12
    // 4,1
    // 0,13
    // 10,12
    // 3,4
    // 3,0
    // 8,4
    // 1,10
    // 2,14
    // 8,10
    // 9,0

    // fold along y=7
    // fold along x=5";
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots = dots
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds = folds
        .lines()
        .map(|l| l.split_once("=").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    println!("{:?}", folds);
    (dots, folds)
}

fn fold(dots: &HashSet<Coord>, (axis, level): &(Axis, u32)) -> HashSet<Coord> {
    match axis {
        Axis::X => fold_x(dots, level),
        Axis::Y => fold_y(dots, level),
    }
}

fn fold_x(dots: &HashSet<Coord>, level: &u32) -> HashSet<Coord> {
    let mut new_dots = HashSet::new();

    for &(x, y) in dots {
        if x < *level {
            new_dots.insert((x, y));
        } else {
            new_dots.insert((2 * *level - x, y));
        }
    }
    new_dots
}

fn fold_y(dots: &HashSet<Coord>, level: &u32) -> HashSet<Coord> {
    let mut new_dots = HashSet::new();

    for &(x, y) in dots {
        if y < *level {
            new_dots.insert((x, y));
        } else {
            new_dots.insert((x, 2 * *level - y));
        }
    }
    new_dots
}

#[aoc(day13, part1)]
pub fn part1((dots, folds): &(HashSet<Coord>, Vec<(Axis, u32)>)) -> usize {
    let first_fold = folds.first().unwrap();
    fold(dots, first_fold).len()
}

