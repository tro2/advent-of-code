use day_01;

#[test]
fn d1_p1_example() {
    assert_eq!(day_01::part_01("tests/example.txt"), 11);
}

#[test]
fn d1_p1_input() {
    assert_eq!(day_01::part_01("tests/input.txt"), 2176849);
}

#[test]
fn d1_p2_example() {
    assert_eq!(day_01::part_02("tests/example.txt"), 31);
}

#[test]
fn d1_p2_input() {
    assert_eq!(day_01::part_02("tests/input.txt"), 23384288);
}
