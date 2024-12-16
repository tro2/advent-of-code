use day_13;

#[test]
fn d13_p1_example() {
    assert_eq!(day_13::part_01("tests/example.txt"), 480)
}

#[test]
fn d13_p1_input() {
    assert_eq!(day_13::part_01("tests/input.txt"), 5208)
}

#[test]
fn d13_p2_example() {
    assert_eq!(day_13::part_02("tests/example.txt"), 6)
}

#[test]
fn d13_p2_input() {
    assert_eq!(day_13::part_02("tests/input.txt"), 1972)
}
