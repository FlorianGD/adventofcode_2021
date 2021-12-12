use std::collections::HashMap;

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    //     let input = "\
    // start-A
    // start-b
    // A-c
    // A-b
    // b-d
    // A-end
    // b-end";
    let edges: Vec<(String, String)> = input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();
    let mut adjacent = HashMap::new();
    edges.iter().for_each(|(a, b)| {
        adjacent
            .entry(a.clone())
            .or_insert(Vec::new())
            .push(b.clone());
        adjacent
            .entry(b.clone())
            .or_insert(Vec::new())
            .push(a.clone());
    });
    adjacent
}

fn is_small(s: &String) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn visit(visited: Vec<String>, adjacent: &HashMap<String, Vec<String>>) -> u32 {
    let current = visited.last().unwrap();
    let possibles = &adjacent[current];
    let mut count = 0;
    for possible in possibles {
        if possible == "end" {
            count += 1;
        } else if is_small(possible) && visited.contains(&possible) {
            // cannot visit this node
            continue;
        } else {
            let mut next = visited.clone();
            next.push(possible.clone());
            count += visit(next, adjacent);
        }
    }
    count
}

#[aoc(day12, part1)]
pub fn part1(adjacent: &HashMap<String, Vec<String>>) -> u32 {
    let mut visited = Vec::new();
    visited.push("start".to_string());
    visit(visited.clone(), adjacent)
}
