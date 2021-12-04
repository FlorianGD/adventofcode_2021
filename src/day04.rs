use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Board {
    pub values: [[u8; 5]; 5],
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .trim_end()
            .split('\n')
            .map(|row| {
                row.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[u8; 5]>>()
            .try_into()
            .unwrap();
        Ok(Board { values })
    }
}
impl Board {
    pub fn win_in(&self, draws: &[u8]) -> usize {
        let min_rows = rows_win_in(&self.values, &draws);
        let cols = transpose(&self.values);
        let min_cols = rows_win_in(&cols, &draws);
        min_rows.min(min_cols)
    }
    pub fn score(&self, draws: &[u8]) -> u32 {
        let sum: u32 = self
            .values
            .into_iter()
            .flatten()
            .filter(|val| !draws.contains(val))
            .map(|x| x as u32)
            .sum();
        sum * (*draws.last().unwrap() as u32)
    }
}

fn transpose(values: &[[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut cols: [[u8; 5]; 5] = [[0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            cols[i][j] = values[j][i]
        }
    }
    cols
}

fn rows_win_in(values: &[[u8; 5]; 5], draws: &[u8]) -> usize {
    values
        .iter()
        .map(|row| {
            row.map(|val| draws.iter().position(|&draw| draw == val).unwrap())
                .into_iter()
                .max()
                .unwrap()
        })
        .into_iter()
        .min()
        .unwrap()
}

pub fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let draws = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let boards = input
        .split("\n\n")
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    (draws, boards)
}

pub fn part1(input: &(Vec<u8>, Vec<Board>)) -> u32 {
    let (draws, boards) = input;
    let (idx_board, num_steps) = boards
        .iter()
        .map(|board| board.win_in(&draws))
        .enumerate()
        .min_by_key(|&(_, val)| val)
        .unwrap();
    boards[idx_board].score(&draws[..=num_steps])
}

pub fn part2(input: &(Vec<u8>, Vec<Board>)) -> u32 {
    let (draws, boards) = input;
    let (idx_board, num_steps) = boards
        .iter()
        .map(|board| board.win_in(&draws))
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .unwrap();
    boards[idx_board].score(&draws[..=num_steps])
}
