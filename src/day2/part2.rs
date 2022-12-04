use super::part1::*;
use std::fs::read_to_string;

fn parse_strategy_guide(input: &str) -> Vec<(HandShape, Outcome)> {
    let mut guide: Vec<(HandShape, Outcome)> = vec![];

    for line in input.lines() {
        let opponent_move = line.chars().nth(0).unwrap();
        let outcome = line.chars().nth(2).unwrap();
        guide.push((
            opponent_hand_shape(opponent_move).unwrap(),
            read_outcome(outcome).unwrap(),
        ));
    }

    guide
}

fn read_outcome(outcome: char) -> Option<Outcome> {
    match outcome {
        'X' => Some(Outcome::Loss),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => None,
    }
}

fn choose_move(opponent: &HandShape, outcome: &Outcome) -> HandShape {
    match (opponent, outcome) {
        (_, Outcome::Draw) => *opponent,
        (HandShape::Rock, Outcome::Win) => HandShape::Paper,
        (HandShape::Paper, Outcome::Win) => HandShape::Scissor,
        (HandShape::Scissor, Outcome::Win) => HandShape::Rock,
        (HandShape::Rock, _) => HandShape::Scissor,
        (HandShape::Paper, _) => HandShape::Rock,
        (HandShape::Scissor, _) => HandShape::Paper,
    }
}

fn follow_guide(guide: Vec<(HandShape, Outcome)>) -> i32 {
    guide.iter().fold(0, |score, (opponent, outcome)| {
        score + *outcome as i32 + choose_move(opponent, outcome) as i32
    })
}

pub fn solve() {
    let filename = "src/day2/input.txt";
    let input = read_to_string(filename).unwrap();

    let strategy_guide = parse_strategy_guide(&input);

    println!(
        "PROPER STRATEGY GUIDE OUTCOME: {}",
        follow_guide(strategy_guide)
    );
}
