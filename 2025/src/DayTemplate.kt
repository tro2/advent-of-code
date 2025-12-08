abstract class DayTemplate<T>(private val fileName: String) {

    protected fun data() = this::class.java.getResourceAsStream(fileName)
        ?: error("Could not load resource $fileName")

    abstract fun part1(): T
    abstract fun part2(): T
}
