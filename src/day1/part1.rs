use std::fs::read_to_string;

pub fn parse_food_packs(input: &str) -> Vec<Vec<i32>> {
    let mut food_packs: Vec<Vec<i32>> = vec![vec![]];

    for line in input.lines() {
        if line.is_empty() {
            food_packs.push(vec![]);
        } else {
            let calories = line.parse::<i32>().unwrap();
            let food_pack = food_packs.last_mut().unwrap();
            food_pack.push(calories);
        }
    }

    food_packs
}

pub fn most_calories_carried(input: Vec<Vec<i32>>) -> i32 {
    input.iter().map(calories_carried).max().unwrap()
}

pub fn calories_carried(food_pack: &Vec<i32>) -> i32 {
    food_pack.iter().sum()
}

pub fn solve() {
    let filename = "src/day1/input.txt";
    let input = read_to_string(filename).unwrap();

    let food_packs = parse_food_packs(&input);

    println!(
        "MOST CALORIES CARRIED: {}",
        most_calories_carried(food_packs)
    );
}
