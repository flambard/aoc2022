#[test]
fn test_parse_input() {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    let food_packs = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];

    let parsed_food_packs = super::part1::parse_food_packs(input);

    assert_eq!(food_packs, parsed_food_packs)
}

#[test]
fn test_elf_carrying_most_calories() {
    let input = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];

    let most_calories = super::part1::most_calories_carried(input);
    assert_eq!(24000, most_calories)
}

#[test]
fn test_part1_solution() {
    super::part1::solve();
}

#[test]
fn test_part2_solution() {
    super::part2::solve();
}
