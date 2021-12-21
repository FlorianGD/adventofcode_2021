use std::collections::HashMap;

type Coord = (isize, isize);
type Matrix = HashMap<Coord, isize>;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Matrix {
    // let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n";
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

fn find_basin_size(matrix: &mut Matrix, start: &Coord) -> u32 {
    let mut acc = 0;
    let &(i, j) = start;
    let neighbors_coords = [&(i, j - 1), &(i - 1, j), &(i + 1, j), &(i, j + 1)];
    for n_coord in neighbors_coords {
        let neighbor_val = matrix.get(n_coord).unwrap_or(&9);
        if neighbor_val == &9 {
            continue;
        }
        matrix.insert(*n_coord, 9);
        acc += 1 + find_basin_size(matrix, n_coord);
    }
    acc
}

#[aoc(day9, part2)]
pub fn part2(input_matrix: &Matrix) -> u32 {
    let coords: Vec<&Coord> = input_matrix.keys().collect();
    let mut matrix = input_matrix.clone();
    let mut basins_sizes = Vec::new();
    // while we have something to explore (i.e. something has a value != 9)
    while matrix.keys().any(|coord| matrix[coord] != 9) {
        let basin_start = coords.iter().find(|&x| matrix[x] != 9).unwrap();

        basins_sizes.push(find_basin_size(&mut matrix, basin_start));
    }
    basins_sizes.sort_unstable();
    basins_sizes.iter().rev().take(3).product()
}
