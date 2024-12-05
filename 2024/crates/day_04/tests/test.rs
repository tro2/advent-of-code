use day_04;

#[test]
fn d4_p1_sm_example() {
    assert_eq!(day_04::part_01("tests/sm_ex.txt"), 4);
}

#[test]
fn d4_p1_dots_example() {
    assert_eq!(day_04::part_01("tests/ex_dots.txt"), 18);
}

#[test]
fn d4_p1_example() {
    assert_eq!(day_04::part_01("tests/example.txt"), 18);
}

#[test]
fn d4_p1_input() {
    assert_eq!(day_04::part_01("tests/input.txt"), 2547);
}
