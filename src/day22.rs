use itertools::iproduct;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

type Cuboid = ((isize, isize), (isize, isize), (isize, isize));
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
                    (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                    (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
                    (cap[6].parse().unwrap(), cap[7].parse().unwrap()),
                ),
            )
        })
        .collect()
}

fn range_in_box(range: &(isize, isize), r_min: &isize, r_max: &isize) -> RangeInclusive<isize> {
    (range.0.max(*r_min))..=(range.1.min(*r_max))
}

#[aoc(day22, part1)]
pub fn part1(steps: &[(bool, Cuboid)]) -> usize {
    const R_MIN: isize = -50;
    const R_MAX: isize = 50;
    let mut cubes = HashMap::new();
    for (is_on, (range_x, range_y, range_z)) in steps.iter() {
        let range_x = range_in_box(&range_x, &R_MIN, &R_MAX);
        let range_y = range_in_box(&range_y, &R_MIN, &R_MAX);
        let range_z = range_in_box(&range_z, &R_MIN, &R_MAX);
        for (x, y, z) in iproduct!(range_x, range_y, range_z) {
            *cubes.entry((x, y, z)).or_insert(false) = *is_on;
        }
    }
    cubes.values().filter(|&x| *x).count()
}

fn intersection(cube1: &Cuboid, cube2: &Cuboid) -> Option<Cuboid> {
    let ((x_min1, x_max1), (y_min1, y_max1), (z_min1, z_max1)) = *cube1;
    let ((x_min2, x_max2), (y_min2, y_max2), (z_min2, z_max2)) = *cube2;
    let x_min = x_min1.max(x_min2);
    if !((x_min1..=x_max1).contains(&x_min) && (x_min2..=x_max2).contains(&x_min)) {
        return None;
    }
    let x_max = x_max1.min(x_max2);
    if !((x_min1..=x_max1).contains(&x_max) && (x_min2..=x_max2).contains(&x_max)) {
        return None;
    }
    let y_min = y_min1.max(y_min2);
    if !((y_min1..=y_max1).contains(&y_min) && (y_min2..=y_max2).contains(&y_min)) {
        return None;
    }
    let y_max = y_max1.min(y_max2);
    if !((y_min1..=y_max1).contains(&y_max) && (y_min2..=y_max2).contains(&y_max)) {
        return None;
    }
    let z_min = z_min1.max(z_min2);
    if !((z_min1..=z_max1).contains(&z_min) && (z_min2..=z_max2).contains(&z_min)) {
        return None;
    }
    let z_max = z_max1.min(z_max2);
    if !((z_min1..=z_max1).contains(&z_max) && (z_min2..=z_max2).contains(&z_max)) {
        return None;
    }
    Some(((x_min, x_max), (y_min, y_max), (z_min, z_max)))
}

fn intersect_on(cube1: &Cuboid, cubes: &mut Vec<(Cuboid, isize)>) {
    let mut to_add = Vec::new();
    cubes.iter().for_each(|(cube2, value)| {
        if cube1 != cube2 {
            if let Some(cube) = intersection(&cube1, &cube2) {
                to_add.push((cube, -value));
            }
        }
    });
    for cube_value in to_add {
        cubes.push(cube_value);
    }
}

fn volume(((x_min, x_max), (y_min, y_max), (z_min, z_max)): &Cuboid) -> isize {
    (x_max - x_min + 1).abs() * (y_max - y_min + 1).abs() * (z_max - z_min + 1).abs()
}

#[aoc(day22, part2)]
pub fn part2(steps: &[(bool, Cuboid)]) -> isize {
    let mut cubes = Vec::new();
    // 20 to check if it works as with part 1
    for &(is_on, cube) in steps.iter() {
        if is_on {
            cubes.push((cube, 1));
            intersect_on(&cube, &mut cubes)
        } else {
            intersect_on(&cube, &mut cubes)
        }
    }
    // println!("{:?}", cubes);
    cubes.iter().map(|(cube, value)| value * volume(cube)).sum()
}
