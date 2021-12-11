use std::collections::{HashMap, HashSet};

type Coord = (i8, i8);

fn neighbors(c: &Coord) -> HashSet<Coord> {
    let mut n = HashSet::new();
    for a in [-1, 0, 1] {
        for b in [-1, 0, 1] {
            if a == 0 && b == 0 {
                continue;
            }
            let i = c.0 + a;
            let j = c.1 + b;

            if i < 0 || i > 9 || j < 0 || j > 9 {
                continue;
            }
            n.insert((i, j));
        }
    }
    n
}

#[derive(Debug, Clone, Copy)]
pub struct Fish {
    pub level: u32,
    pub flashed: bool,
}

impl Fish {
    fn new(val: char) -> Fish {
        Fish {
            level: val.to_digit(10).unwrap(),
            flashed: false,
        }
    }
    fn update(&mut self) -> bool {
        if self.flashed {
            return false;
        }
        self.level += 1;
        if self.level == 10 {
            self.flashed = true;
            return true;
        }
        false
    }
    fn reset(&mut self) {
        if self.flashed {
            self.level = 0;
        }
        self.flashed = false;
    }
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> HashMap<Coord, Fish> {
    //     let input = "\
    // 5483143223
    // 2745854711
    // 5264556173
    // 6141336146
    // 6357385478
    // 4167524645
    // 2176841721
    // 6882881134
    // 4846848554
    // 5283751526
    // ";
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().enumerate() {
            map.insert(
                (i.try_into().unwrap(), j.try_into().unwrap()),
                Fish::new(val),
            );
        }
    }
    map
}

fn step(map: &mut HashMap<Coord, Fish>) -> u32 {
    let mut count = 0;

    let mut to_update = Vec::new();
    // 1st pass update all, add neighbors to to_update
    for (c, f) in map.iter_mut() {
        let flashed = f.update();
        if flashed {
            to_update.push(*c);
        }
    }

    while !to_update.is_empty() {
        count += 1;
        let c = to_update.pop().unwrap();
        for n in neighbors(&c) {
            let flashed = map.get_mut(&n).unwrap().update();
            if flashed {
                to_update.push(n);
            }
        }
    }

    // 3rd set flashed to false everywhere
    map.values_mut().for_each(|f| f.reset());

    count
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<Coord, Fish>) -> u32 {
    let mut map = input.clone();

    (0..100).map(|_| step(&mut map)).sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &HashMap<Coord, Fish>) -> u32 {
    let mut map = input.clone();
    let mut time = 0;
    while !map.values().all(|f| f.level == 0) {
        step(&mut map);
        time += 1;
    }
    time
}
