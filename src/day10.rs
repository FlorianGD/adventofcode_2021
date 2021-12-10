use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<String> {
    let input ="\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    input.lines().map(|x| x.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn part1(lines : &Vec<String>) -> u32 {
    for line in lines {
        let mut parens = HashMap::new();
        for (i, c) in line.chars().enumerate() {
            match c {
                '{' | '(' | '[' | '<' => *parens.entry(c).or_insert(0) +=1,
                '}' => *parens.entry('{').or_insert(0) -=1,
                ')' => *parens.entry('(').or_insert(0) -=1,
                ']' => *parens.entry('[').or_insert(0) -=1,
                '>' => *parens.entry('<').or_insert(0) -=1,
                _ => unreachable!(),
            }
        }

        println!("{:?}", parens);
    }
    0
}

