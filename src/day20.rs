use std::collections::HashMap;

type Coord = (isize, isize);

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> (HashMap<usize, bool>, HashMap<Coord, bool>) {
    //     let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    // #..#.
    // #....
    // ##..#
    // ..#..
    // ..###";
    let enh = input.lines().next().unwrap();
    let mut dict = HashMap::new();
    for (i, c) in enh.chars().enumerate() {
        dict.insert(i, if c == '#' { true } else { false });
    }

    let mut coords = HashMap::new();
    for (i, line) in input.lines().skip(2).enumerate() {
        for (j, c) in line.chars().enumerate() {
            coords.insert(
                (i as isize, j as isize),
                if c == '#' { true } else { false },
            );
        }
    }
    (dict, coords)
}

fn get_neighbors((i, j): &Coord) -> [Coord; 9] {
    let mut neighbors = Vec::new();
    for a in [-1, 0, 1] {
        for b in [-1, 0, 1] {
            neighbors.push((i + a, j + b));
        }
    }
    neighbors.try_into().unwrap()
}

fn compute_val(vec: &[bool]) -> usize {
    let mut value = 0;
    for (i, b) in vec.iter().rev().enumerate() {
        if *b {
            value += 2_usize.pow(i.try_into().unwrap());
        }
    }
    value
}

fn new_val(c: &Coord, state: &HashMap<Coord, bool>, default: &bool) -> usize {
    let neighbors = get_neighbors(c);
    let mut vec = Vec::new();
    for n in neighbors.iter() {
        let &val = state.get(&n).unwrap_or(default);
        vec.push(val);
    }
    compute_val(&vec)
}

fn display(state: &HashMap<Coord, bool>) {
    let &x_min = state.keys().map(|(x, _)| x).min().unwrap();
    let &x_max = state.keys().map(|(x, _)| x).max().unwrap();
    let &y_min = state.keys().map(|(_, y)| y).min().unwrap();
    let &y_max = state.keys().map(|(_, y)| y).max().unwrap();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if *state.get(&(x, y)).unwrap_or(&false) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}

fn step(
    dict: &HashMap<usize, bool>,
    coords: &HashMap<Coord, bool>,
    default: &bool,
) -> HashMap<Coord, bool> {
    let mut new_state = coords.clone();
    let &x_min = coords.keys().map(|(x, _)| x).min().unwrap();
    let &x_max = coords.keys().map(|(x, _)| x).max().unwrap();
    let &y_min = coords.keys().map(|(_, y)| y).min().unwrap();
    let &y_max = coords.keys().map(|(_, y)| y).max().unwrap();

    for x in x_min - 1..=x_max + 1 {
        for y in y_min - 1..=y_max + 1 {
            let val = new_val(&(x, y), coords, &default);
            let new = dict[&val];
            new_state.insert((x, y), new);
            // *new_state.entry((x, y)).or_insert(new) = new;
        }
    }
    new_state
}

#[aoc(day20, part1)]
pub fn part1((dict, coords): &(HashMap<usize, bool>, HashMap<Coord, bool>)) -> usize {
    // display(&coords);
    let mut default = &false;
    let mut new_state = coords.clone();
    for _ in 0..2 {
        new_state = step(dict, &new_state, &default);
        default = &dict[&compute_val(&[*default; 9])];
        // display(&new_state);
    }
    // println!();
    new_state.values().filter(|&x| *x).count()
}

#[aoc(day20, part2)]
pub fn part2((dict, coords): &(HashMap<usize, bool>, HashMap<Coord, bool>)) -> usize {
    // display(&coords);
    let mut default = &false;
    let mut new_state = coords.clone();
    for _ in 0..50 {
        new_state = step(dict, &new_state, &default);
        default = &dict[&compute_val(&[*default; 9])];
        // display(&new_state);
    }
    // println!();
    new_state.values().filter(|&x| *x).count()
}
