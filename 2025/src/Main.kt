fun main() {
    val d1Ex = Day1("Day1_ex.txt")
    if (d1Ex.test1(3)) {
        println("Day 1 test passed")
    } else {
        println("Day 1 test failed")
    }

    val d1Input = Day1("Day1_input.txt")
    val output = d1Input.part1()
    if (d1Input.part1() == 1191) {
        println("Day 1 input test passed")
    } else {
        println("Day 1 input test failed, output: $output")
    }
}