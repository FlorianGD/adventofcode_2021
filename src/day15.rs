use petgraph::algo::astar;
use petgraph::graphmap::DiGraphMap;
use std::collections::{HashMap, HashSet};

type Coord = (isize, isize);

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> HashMap<Coord, u32> {
    //     let input = "\
    // 1163751742
    // 1381373672
    // 2136511328
    // 3694931569
    // 7463417111
    // 1319128137
    // 1359912421
    // 3125421639
    // 1293138521
    // 2311944581";
    let mut hm = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            hm.insert((i as isize, j as isize), c.to_digit(10).unwrap());
        })
    });
    hm
}

fn neighbors(c: &Coord, x_max: isize, y_max: isize) -> HashSet<Coord> {
    let mut n = HashSet::new();
    for a in [-1, 0, 1] {
        for b in [-1, 0, 1] {
            if a == b || a == -b {
                continue;
            }
            let i = c.0 + a;
            let j = c.1 + b;

            if i < 0 || i > x_max || j < 0 || j > y_max {
                continue;
            }
            n.insert((i, j));
        }
    }
    n
}

fn build_graph(coords: &HashMap<Coord, u32>) -> DiGraphMap<Coord, u32> {
    let mut graph = DiGraphMap::new();
    let x_max = coords.keys().map(|c| c.0).max().unwrap();
    let y_max = coords.keys().map(|c| c.1).max().unwrap();
    coords.keys().for_each(|&c| {
        for n in neighbors(&c, x_max, y_max) {
            graph.add_edge(c, n, coords[&n]);
            graph.add_edge(n, c, coords[&c]);
        }
    });
    graph
}

#[aoc(day15, part1)]
pub fn part1(coords: &HashMap<Coord, u32>) -> u32 {
    let x_max = coords.keys().map(|c| c.0).max().unwrap();
    let y_max = coords.keys().map(|c| c.1).max().unwrap();

    let graph = build_graph(&coords);
    let origin = (0, 0);
    let dest = (x_max, y_max);

    let (cost, _path) = astar(
        &graph,
        origin,
        |finish| finish == dest,
        |(_from, _to, &weight)| weight,
        |_| 0,
    )
    .unwrap();
    cost
}
