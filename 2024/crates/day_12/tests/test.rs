use day_12;

#[test]
fn d12_p1_example() {
    assert_eq!(day_12::part_01("tests/example.txt"), 1930)
}

#[test]
fn d12_p1_input() {
    assert_eq!(day_12::part_01("tests/input.txt"), 1370100)
}

#[test]
fn d12_p2_example() {
    assert_eq!(day_12::part_02("tests/example.txt"), 1206)
}

#[test]
fn d12_p2_input() {
    assert_eq!(day_12::part_02("tests/input.txt"), 810061)
}
