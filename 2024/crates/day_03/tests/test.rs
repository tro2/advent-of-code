use std::fs::read_to_string;

#[test]
fn d3_p1_example() {
    let input = read_to_string("tests/example.txt").expect("Failed to read input file");

    let res = day_03::part_01(&input).unwrap_or_else(|e| panic!("part_01 failed: {e}"));

    assert_eq!(res, 161);
}

#[test]
fn d3_p1_input() {
    let input = read_to_string("tests/input.txt").expect("Failed to read input file");

    let res = day_03::part_01(&input).unwrap_or_else(|e| panic!("part_01 failed: {e}"));

    assert_eq!(res, 160_672_468);
}

#[test]
fn d3_p2_example() {
    let input = read_to_string("tests/ex_p2.txt").expect("Failed to read input file");

    let res = day_03::part_02(&input).unwrap_or_else(|e| panic!("part_02 failed: {e}"));

    assert_eq!(res, 48);
}

#[test]
fn d3_p2_input() {
    let input = read_to_string("tests/input.txt").expect("Failed to read input file");

    let res = day_03::part_02(&input).unwrap_or_else(|e| panic!("part_02 failed: {e}"));

    assert_eq!(res, 84_893_551);
}
