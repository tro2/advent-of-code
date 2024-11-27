use day_04;

#[test]
fn d4_part_01_example() {
    assert_eq!(day_04::part_01("tests/day_04_example.txt"), 13);
}

#[test]
fn d4_part_01_input() {
    assert_eq!(day_04::part_01("tests/day_04_input.txt"), 20667);
}