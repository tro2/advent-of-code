use day_07;

#[test]
fn d7_p1_example() {
    assert_eq!(day_07::part_01("tests/example.txt"), 3749)
}

#[test]
fn d7_p1_input() {
    assert_eq!(day_07::part_01("tests/input.txt"), 5208)
}

#[test]
fn d7_p2_example() {
    assert_eq!(day_07::part_02("tests/example.txt"), 6)
}

#[test]
fn d7_p2_input() {
    assert_eq!(day_07::part_02("tests/input.txt"), 1972)
}
