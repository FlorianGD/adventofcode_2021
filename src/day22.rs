use itertools::iproduct;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

type Cuboid = (
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
);
// on x=-13..37,y=-36..14,z=-3..45
// on x=-77707..-66888,y=-15495..7977,z=21658..44356

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> Vec<(bool, Cuboid)> {
    lazy_static! {
        static ref STEP: Regex =
            Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
                .unwrap();
    }
    input
        .lines()
        .map(|line| STEP.captures(line).unwrap())
        .map(|cap| {
            (
                &cap[1] == "on",
                (
                    cap[2].parse().unwrap()..=cap[3].parse().unwrap(),
                    cap[4].parse().unwrap()..=cap[5].parse().unwrap(),
                    cap[6].parse().unwrap()..=cap[7].parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn range_in_box(
    range: &RangeInclusive<isize>,
    r_min: &isize,
    r_max: &isize,
) -> RangeInclusive<isize> {
    (*range.start().max(r_min))..=(*range.end().min(r_max))
}

#[aoc(day22, part1)]
pub fn part1(steps: &[(bool, Cuboid)]) -> usize {
    const R_MIN: isize = -50;
    const R_MAX: isize = 50;
    let mut cubes = HashMap::new();
    for (is_on, (range_x, range_y, range_z)) in steps {
        let range_x = range_in_box(&range_x, &R_MIN, &R_MAX);
        let range_y = range_in_box(&range_y, &R_MIN, &R_MAX);
        let range_z = range_in_box(&range_z, &R_MIN, &R_MAX);
        for (x, y, z) in iproduct!(range_x, range_y, range_z) {
            *cubes.entry((x, y, z)).or_insert(false) = *is_on;
        }
    }
    cubes.values().filter(|&x| *x).count()
}
