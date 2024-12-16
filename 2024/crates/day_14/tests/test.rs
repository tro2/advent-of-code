use day_14;

#[test]
fn d14_p1_example() {
    assert_eq!(day_14::part_01("tests/example.txt"), 12)
}

#[test]
fn d14_p1_input() {
    assert_eq!(day_14::part_01("tests/input.txt"), 224554908)
}

#[test]
fn d14_p2_input() {
    assert_eq!(day_14::part_02("tests/input.txt"), 0) // always succeeds, generates output.txt
}
