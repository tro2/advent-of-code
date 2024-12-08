use day_00;

#[test]
fn d0_p1_example() {
    assert_eq!(day_00::part_01("tests/example.txt"), 3749)
}

#[test]
fn d0_p1_input() {
    assert_eq!(day_00::part_01("tests/input.txt"), 5208)
}

#[test]
fn d0_p2_example() {
    assert_eq!(day_00::part_02("tests/example.txt"), 6)
}

#[test]
fn d0_p2_input() {
    assert_eq!(day_00::part_02("tests/input.txt"), 1972)
}
