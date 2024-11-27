use day_02;

#[test]
fn d2_part_01_example() {
    assert_eq!(day_02::part_01("tests/day_02_example.txt"), 8);
}

#[test]
fn d2_part_01_input() {
    assert_eq!(day_02::part_01("tests/day_02_input.txt"), 2541);
}

#[test]
fn d2_part_02_example() {
    assert_eq!(day_02::part_02("tests/day_02_example.txt"), 2286);
}

#[test]
fn d2_part_02_input() {
    assert_eq!(day_02::part_02("tests/day_02_input.txt"), 66016);
}