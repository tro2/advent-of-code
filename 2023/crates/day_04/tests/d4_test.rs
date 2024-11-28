use day_04;

#[test]
fn d4_part_01_example() {
    assert_eq!(day_04::part_01("tests/d4_example.txt"), 13);
}

#[test]
fn d4_part_01_input() {
    assert_eq!(day_04::part_01("tests/d4_input.txt"), 20667);
}

#[test]
fn d4_part_02_example() {
    assert_eq!(day_04::part_02("tests/d4_example.txt"), 30);
}

#[test]
fn d4_part_02_input() {
    assert_eq!(day_04::part_02("tests/d4_input.txt"), 5833065);
}