use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<String> {
    //     let input = "\
    // [({(<(())[]>[[{[]{<()<>>
    // [(()[<>])]({[<{<<[]>>(
    // {([(<{}[<>[]}>{[]{[(<()>
    // (((({<>}<{<{<>}{[]{[]{}
    // [[<[([]))<([[{}[[()]]]
    // [{[{({}]{}}([{[{{{}}([]
    // {<[[]]>}<{[{[{[]{()[[[]
    // [<(<(<(<{}))><([]([]()
    // <{([([[(<>()){}]>(<<{{
    // <{([{{}}[<[[[<>{}]]]>[]]
    // ";
    input.lines().map(|x| x.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn part1(lines: &Vec<String>) -> u32 {
    let mut score = 0;
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let closing_to_opening = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    for line in lines {
        let mut scope = Vec::new();
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => {
                    scope.push(c);
                }
                '}' | ')' | ']' | '>' => {
                    let current = scope.pop();
                    match current {
                        Some(v) => {
                            if closing_to_opening[&c] != v {
                                score += points[&c];
                                break;
                            }
                        }
                        None => {
                            score += points[&c];
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    score
}

#[aoc(day10, part2)]
pub fn part2(lines: &Vec<String>) -> u64 {
    let mut scores = Vec::new();
    let closing_to_opening = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let opening_to_closing = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    for line in lines {
        let mut scope = Vec::new();
        let mut skip = false;
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => {
                    scope.push(c);
                }
                '}' | ')' | ']' | '>' => {
                    let current = scope.pop();
                    match current {
                        Some(v) => {
                            if closing_to_opening[&c] != v {
                                skip = true;
                                break;
                            }
                        }
                        None => {
                            skip = true;
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        if skip {
            continue;
        };
        let mut score = 0;
        for remaining in scope.iter().rev() {
            let point = points[&opening_to_closing[&remaining]];
            score *= 5;
            score += point;
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}
