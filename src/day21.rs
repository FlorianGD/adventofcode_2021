use std::collections::HashMap;

#[derive(Debug)]
struct DeterministicDie {
    num_rolled: usize,
    next_roll: usize,
}

impl DeterministicDie {
    pub fn new() -> DeterministicDie {
        DeterministicDie {
            num_rolled: 0,
            next_roll: 1,
        }
    }
    pub fn roll(&mut self) -> usize {
        self.num_rolled += 1;
        let roll = self.next_roll;
        self.next_roll = (self.next_roll % 100) + 1;
        roll
    }
}

#[derive(Debug, Clone)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    pub fn new(start_position: usize) -> Player {
        Player {
            position: start_position,
            score: 0,
        }
    }
    pub fn play(&mut self, die: &mut DeterministicDie) -> usize {
        let rolls = die.roll() + die.roll() + die.roll();
        self.update_score(rolls)
    }
    pub fn update_score(&mut self, rolls: usize) -> usize {
        self.position = (self.position + rolls - 1) % 10 + 1;
        self.score += self.position;
        self.score
    }
}

#[aoc(day21, part1)]
pub fn part1(_input: &str) -> usize {
    let mut die = DeterministicDie::new();
    // let mut players = [Player::new(4), Player::new(8)];
    let mut players = [Player::new(5), Player::new(6)];
    let mut scores = [players[0].score, players[1].score];
    let mut i = 0;
    loop {
        scores[i] = players[i].play(&mut die);
        if scores[i] >= 1000 {
            break;
        }
        i = (i + 1) % 2;
    }
    die.num_rolled * players[(i + 1) % 2].score
}

#[aoc(day21, part2)]
pub fn part2(_input: &str) -> usize {
    // let player1 = Player::new(4);
    // let player2 = Player::new(8);
    let player1 = Player::new(5);
    let player2 = Player::new(6);
    // The possibilities after rolling 3 dice (value and count)
    let possible_rolls: HashMap<usize, usize> =
        HashMap::from_iter([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);

    let mut wins = HashMap::new();
    plays_dirac(&player1, &player2, true, 1, &mut wins, &possible_rolls);
    *wins.values().max().unwrap()
}

fn plays_dirac(
    player1: &Player,
    player2: &Player,
    p1_plays: bool,
    times: usize,
    wins: &mut HashMap<usize, usize>,
    possible_rolls: &HashMap<usize, usize>,
) {
    if p1_plays {
        for (rolls, time) in possible_rolls {
            let mut player = player1.clone();
            player.update_score(*rolls);
            if player.score >= 21 {
                *wins.entry(1).or_insert(0) += times * time;
                continue;
            }
            plays_dirac(
                &player,
                &player2,
                !p1_plays,
                time * times,
                wins,
                possible_rolls,
            );
        }
    } else {
        for (rolls, time) in possible_rolls {
            let mut player = player2.clone();
            player.update_score(*rolls);
            if player.score >= 21 {
                *wins.entry(2).or_insert(0) += times * time;
                continue;
            }
            plays_dirac(
                &player1,
                &player,
                !p1_plays,
                time * times,
                wins,
                possible_rolls,
            );
        }
    }
}
