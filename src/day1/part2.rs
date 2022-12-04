use std::fs::read_to_string;

use super::part1::*;

pub fn solve() {
    let filename = "src/day1/input.txt";
    let input = read_to_string(filename).unwrap();

    let food_packs = parse_food_packs(&input);

    let mut calorie_counts = food_packs
        .iter()
        .map(calories_carried)
        .collect::<Vec<i32>>();

    calorie_counts.sort();
    calorie_counts.reverse();

    let top3 = &calorie_counts[0..3];

    println!("TOP 3 SUM: {}", top3.iter().sum::<i32>());
}
