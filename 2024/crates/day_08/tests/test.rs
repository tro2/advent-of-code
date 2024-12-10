use day_08;

#[test]
fn d8_p1_simple() {
    assert_eq!(day_08::part_01("tests/simple.txt"), 4)
}

#[test]
fn d8_p1_example() {
    assert_eq!(day_08::part_01("tests/example.txt"), 14)
}

#[test]
fn d8_p1_input() {
    assert_eq!(day_08::part_01("tests/input.txt"), 271)
}

#[test]
fn d8_p2_example() {
    assert_eq!(day_08::part_02("tests/example.txt"), 34)
}

#[test]
fn d8_p2_input() {
    assert_eq!(day_08::part_02("tests/input.txt"), 994)
}
