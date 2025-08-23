#[test]
fn d10_p1_sm() {
    assert_eq!(day_10::part_01("tests/sm.txt"), 4)
}

#[test]
fn d10_p1_example() {
    assert_eq!(day_10::part_01("tests/example.txt"), 36)
}

#[test]
fn d10_p1_input() {
    assert_eq!(day_10::part_01("tests/input.txt"), 717)
}

#[test]
fn d10_p2_dense() {
    assert_eq!(day_10::part_02("tests/dense.txt"), 227)
}

#[test]
fn d10_p2_example() {
    assert_eq!(day_10::part_02("tests/example.txt"), 81)
}

#[test]
fn d10_p2_input() {
    assert_eq!(day_10::part_02("tests/input.txt"), 1686)
}
