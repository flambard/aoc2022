use super::part1::HandShape;

#[test]
fn test_parse_input() {
    let input = r#"A Y
B X
C Z"#;

    let guide = vec![
        (HandShape::Rock, HandShape::Paper),
        (HandShape::Paper, HandShape::Rock),
        (HandShape::Scissor, HandShape::Scissor),
    ];

    let parsed_guide = super::part1::parse_strategy_guide(input);

    assert_eq!(guide, parsed_guide)
}

#[test]
fn test_part1_solution() {
    super::part1::solve();
}

#[test]
fn test_part2_solution() {
    super::part2::solve();
}
