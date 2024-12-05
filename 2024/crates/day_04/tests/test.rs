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

#[test]
fn d4_p2_ex_dots() {
    assert_eq!(day_04::part_02("tests/ex_dots_2.txt"), 9);
}


#[test]
fn d4_p2_example() {
    assert_eq!(day_04::part_02("tests/example.txt"), 9);
}

#[test]
fn d4_p2_input() {
    assert_eq!(day_04::part_02("tests/input.txt"), 1939);
}
