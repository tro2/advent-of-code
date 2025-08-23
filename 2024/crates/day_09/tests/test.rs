#[test]
fn d9_p1_example() {
    assert_eq!(day_09::part_01("tests/example.txt"), 1928)
}

#[test]
fn d9_p1_input() {
    assert_eq!(day_09::part_01("tests/input.txt"), 6382875730645)
}

#[test]
fn d9_p2_example() {
    assert_eq!(day_09::part_02("tests/example.txt"), 2858)
}

#[test]
fn d9_p2_input() {
    assert_eq!(day_09::part_02("tests/input.txt"), 6420913943576)
}
