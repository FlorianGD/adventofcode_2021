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
        let roll = self.next_roll.clone();
        self.next_roll = (self.next_roll % 100) + 1;
        roll
    }
}

#[derive(Debug)]
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
