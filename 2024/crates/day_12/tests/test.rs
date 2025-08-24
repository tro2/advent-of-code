use std::fs::read_to_string;

#[test]
fn d3_p1_example() {
    let input = read_to_string("tests/example.txt").expect("Failed to read input file");

    let res = day_12::part_01(&input).unwrap_or_else(|e| panic!("part_01 failed: {e}"));

    assert_eq!(res, 1930);
}

#[test]
fn d3_p1_input() {
    let input = read_to_string("tests/input.txt").expect("Failed to read input file");

    let res = day_12::part_01(&input).unwrap_or_else(|e| panic!("part_01 failed: {e}"));

    assert_eq!(res, 1_370_100);
}

#[test]
fn d3_p2_example() {
    let input = read_to_string("tests/example.txt").expect("Failed to read input file");

    let res = day_12::part_02(&input).unwrap_or_else(|e| panic!("part_02 failed: {e}"));

    assert_eq!(res, 1206);
}

#[test]
fn d3_p2_input() {
    let input = read_to_string("tests/input.txt").expect("Failed to read input file");

    let res = day_12::part_02(&input).unwrap_or_else(|e| panic!("part_02 failed: {e}"));

    assert_eq!(res, 818_286);
}
