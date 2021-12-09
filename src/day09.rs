use std::collections::HashMap;

type Matrix = HashMap<(isize, isize), isize>;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Matrix {
    //     let input = "\
    // 2199943210
    // 3987894921
    // 9856789892
    // 8767896789
    // 9899965678
    // ";
    let mut matrix = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, digit) in line.chars().enumerate() {
            matrix.insert(
                (i as isize, j as isize),
                digit.to_digit(10).unwrap() as isize,
            );
        }
    }
    matrix
}

fn below_neighbors(matrix: &Matrix, i: isize, j: isize, val: isize) -> isize {
    let neighbors = [
        matrix.get(&(i, j - 1)).unwrap_or(&10),
        matrix.get(&(i - 1, j)).unwrap_or(&10),
        matrix.get(&(i + 1, j)).unwrap_or(&10),
        matrix.get(&(i, j + 1)).unwrap_or(&10),
    ];

    if neighbors.iter().all(|&n| n > &val) {
        val + 1
    } else {
        0
    }
}

#[aoc(day9, part1)]
pub fn part1(matrix: &Matrix) -> isize {
    matrix
        .iter()
        .map(|(&(i, j), &item)| below_neighbors(matrix, i, j, item))
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(matrix: &Matrix) -> isize {
    matrix
        .iter()
        .map(|(&(i, j), &item)| below_neighbors(matrix, i, j, item))
        .sum()
}
