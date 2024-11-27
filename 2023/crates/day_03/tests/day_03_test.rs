use day_03;

#[test]
fn d3_part_01_example() {
    assert_eq!(day_03::part_01("tests/day_03_example.txt"), 4361);
}

#[test]
fn d3_part_01_input() {
    assert_eq!(day_03::part_01("tests/day_03_input.txt"), 514969);
}

#[test]
fn d3_part_02_example() {
    assert_eq!(day_03::part_02("tests/day_03_example.txt"), 467835);
}

#[test]
fn d3_part_02_input() {
    assert_eq!(day_03::part_02("tests/day_03_input.txt"), 78915902);
}