use day_01;

#[test]
fn part_01() {
    assert_eq!(day_01::part_1("tests/day_01_example.txt"), 142);
    assert_eq!(day_01::part_1("tests/day_01_input.txt"), 55477);
}
#[test]
fn part_02() {
    assert_eq!(day_01::part_2("tests/day_01_example_2.txt"), 281);
    assert_eq!(day_01::part_2("tests/day_01_input.txt"), 54431);
}