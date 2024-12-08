use day_06;

#[test]
fn d6_p1_example() {
    assert_eq!(day_06::part_01("tests/example.txt"), 41)
}

#[test]
fn d6_p1_input() {
    assert_eq!(day_06::part_01("tests/input.txt"), 5208)
}

#[test]
fn d6_p2_example() {
    assert_eq!(day_06::part_02("tests/example.txt"), 6)
}

#[test]
fn d6_p2_input() {
    assert_eq!(day_06::part_02("tests/input.txt"), 1972)
}
