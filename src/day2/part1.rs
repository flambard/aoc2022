use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Clone, Copy)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

pub fn parse_strategy_guide(input: &str) -> Vec<(HandShape, HandShape)> {
    let mut guide: Vec<(HandShape, HandShape)> = vec![];

    for line in input.lines() {
        let opponent_move = line.chars().nth(0).unwrap();
        let your_move = line.chars().nth(2).unwrap();
        guide.push((
            opponent_hand_shape(opponent_move).unwrap(),
            your_hand_shape(your_move).unwrap(),
        ));
    }

    guide
}

pub fn opponent_hand_shape(char_move: char) -> Option<HandShape> {
    match char_move {
        'A' => Some(HandShape::Rock),
        'B' => Some(HandShape::Paper),
        'C' => Some(HandShape::Scissor),
        _ => None,
    }
}

fn your_hand_shape(char_move: char) -> Option<HandShape> {
    match char_move {
        'X' => Some(HandShape::Rock),
        'Y' => Some(HandShape::Paper),
        'Z' => Some(HandShape::Scissor),
        _ => None,
    }
}

fn play_round(opponent: &HandShape, you: &HandShape) -> Outcome {
    if opponent == you {
        Outcome::Draw
    } else {
        match (opponent, you) {
            (HandShape::Rock, HandShape::Paper) => Outcome::Win,
            (HandShape::Paper, HandShape::Scissor) => Outcome::Win,
            (HandShape::Scissor, HandShape::Rock) => Outcome::Win,
            _ => Outcome::Loss,
        }
    }
}

fn follow_guide(guide: Vec<(HandShape, HandShape)>) -> i32 {
    guide.iter().fold(0, |score, (opponent, you)| {
        score + *you as i32 + play_round(opponent, you) as i32
    })
}

pub fn solve() {
    let filename = "src/day2/input.txt";
    let input = read_to_string(filename).unwrap();

    let strategy_guide = parse_strategy_guide(&input);

    println!("STRATEGY GUIDE OUTCOME: {}", follow_guide(strategy_guide));
}
