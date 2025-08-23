#[test]
fn d2_p1_example() {
    assert_eq!(day_02::part_01("tests/example.txt"), 2);
}

#[test]
fn d2_p1_input() {
    assert_eq!(day_02::part_01("tests/input.txt"), 516);
}

#[test]
fn d2_p2_example() {
    assert_eq!(day_02::part_02("tests/example.txt"), 4);
}

#[test]
fn d2_p2_input() {
    assert_eq!(day_02::part_02("tests/input.txt"), 561);
}
