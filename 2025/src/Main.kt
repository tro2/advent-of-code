fun main() {
    // test(::Day1, "Day1_ex.txt", "Day1_input.txt")
    // test(::Day2, "d2ex.txt", "d2input.txt")
    // test(::Day3, "d3ex.txt", "d3input.txt")
    test(::Day4, "d4ex.txt", "d4input.txt")
}

fun <T> test(
    ctor: (String) -> DayTemplate<T>,
    vararg files: String
) {
    for (f in files) {
        val day = ctor(f)
        println("$f / part1 = ${day.part1()}")
        println("$f / part2 = ${day.part2()}")
    }
}