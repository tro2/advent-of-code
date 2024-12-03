use day_03;

#[test]
fn d3_p1_example() {
    assert_eq!(day_03::part_01("tests/example.txt"), 161);
}

#[test]
fn d3_p1_input() {
    assert_eq!(day_03::part_01("tests/input.txt"), 160672468);
}

#[test]
fn d3_p2_example() {
    assert_eq!(day_03::part_02("tests/ex_p2.txt"), 48);
}

#[test]
fn d3_p2_input() {
    assert_eq!(day_03::part_02("tests/input.txt"), 84893551);
}
