use std::fs::read_to_string;

#[test]
fn p1_example() {
    let input = read_to_string("tests/example.txt").expect("Failed to read input file");

    let res = day_01::part_01(&input).unwrap_or_else(|e| panic!("part_01 failed: {e}"));

    assert_eq!(res, 11);
}

#[test]
fn p1_input() {
    let input = read_to_string("tests/input.txt").expect("Failed to read input file");

    let res = day_01::part_01(&input).unwrap_or_else(|e| panic!("part_01 failed: {e}"));

    assert_eq!(res, 2_176_849);
}

#[test]
fn p2_example() {
    let input = read_to_string("tests/example.txt").expect("Failed to read input file");

    let res = day_01::part_02(&input).unwrap_or_else(|e| panic!("part_02 failed: {e}"));

    assert_eq!(res, 31);
}

#[test]
fn p2_input() {
    let input = read_to_string("tests/input.txt").expect("Failed to read input file");

    let res = day_01::part_02(&input).unwrap_or_else(|e| panic!("part_02 failed: {e}"));

    assert_eq!(res, 23_384_288);
}
