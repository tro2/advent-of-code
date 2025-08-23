#[test]
fn d5_p1_example() {
    assert_eq!(day_05::part_01("tests/example.txt"), 143);
}

#[test]
fn d5_p1_input() {
    assert_eq!(day_05::part_01("tests/input.txt"), 5329);
}

#[test]
fn d5_p2_example() {
    assert_eq!(day_05::part_02("tests/example.txt"), 123);
}

#[test]
fn d5_p2_input() {
    assert_eq!(day_05::part_02("tests/input.txt"), 5833);
}
