import java.io.InputStream

abstract class DayTemplate<T>(fileName: String) {

    protected val data: InputStream = this::class.java.getResourceAsStream(fileName)
        ?: error("Could not load resource $fileName")

    abstract fun part1(): T
    abstract fun part2(): T

    fun test1(expected: T): Boolean = part1() == expected
    fun test2(expected: T): Boolean = part2() == expected
}
