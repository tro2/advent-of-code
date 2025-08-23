#[test]
fn d11_p1_sm() {
    assert_eq!(day_11::part_01("tests/sm.txt", 1), 7)
}

#[test]
fn d11_p1_example() {
    assert_eq!(day_11::part_01("tests/example.txt", 25), 55312)
}

#[test]
fn d11_p1_input() {
    assert_eq!(day_11::part_01("tests/input.txt", 25), 203609)
}

#[test]
fn d11_p2_example() {
    assert_eq!(day_11::part_02("tests/example.txt", 25), 55312)
}

#[test]
fn d11_p2_input() {
    assert_eq!(day_11::part_02("tests/input.txt", 75), 240954878211138)
}
