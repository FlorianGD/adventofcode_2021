use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Coord(i8, i8);

impl Coord {
    fn neighbors(&self) -> HashSet<Coord> {
        let mut n = HashSet::new();
        for a in [-1,0,1]  {
            for b in [-1,0,1] {
                if a == 0 && b == 0 {
                    continue
                }
                let i = self.0 + a;
                let j = self.1 + b;

                if i < 0 || i > 9 || j < 0 || j >9 {
                    continue
                }
                n.insert(Coord(i,j));
            }
        }
        n
    }
}

#[derive(Debug, Clone)]
pub struct Fish {
    level: u32,
    flashed: bool
}

impl Fish {
    fn new(val: char) -> Fish {
        Fish { level : val.to_digit(10).unwrap(), flashed: false }
    }
    fn update(&mut self) -> bool {
        if self.flashed {
            return false
        }
        self.level += 1;
        if self.level == 10 {
            self.level = 0;
            self.flashed = true;
            return true
        }
        false
    }
}


#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> HashMap<Coord, Fish> {
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().enumerate() {
            map.insert(Coord(i.try_into().unwrap(),j.try_into().unwrap()), Fish::new(val));
        }
    }
    map
}

fn step(map: &mut HashMap<Coord, Fish>) -> u32 {
    let mut count = 0;
    let mut to_update =Vec::new();
    // 1st pass updqte all, add neighbors to to_updqte
    // 2nd while to_updqte is not empty, pop, updste and queue neihbors
    // 3rd set flashed to false everywgere
    for (c, mut f) in map.into_iter() {
        let flashed = f.update();
        if flashed {
            to_update.push(c);
            count += 1;
        }
    }
    println!("{:?}", map);

    count
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<Coord, Fish>) -> u32 {
    let mut map = input.clone();
    step(&mut map)
}

