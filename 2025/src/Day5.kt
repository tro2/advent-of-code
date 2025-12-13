class Day5(fileName: String) : DayTemplate<Long>(fileName) {

    private val input by lazy {
        data().bufferedReader().readText().splitToSequence("\n\n")
    }

    override fun part1(): Long {
        val (rawRanges, rawIds) = input.toList()

        val freshRanges = rawRanges
            .lineSequence()
            .map { range ->
                val (left, right) = range.split('-').map(String::toLong)
                left..right
            }
            .mergeRanges()

        return rawIds
            .lineSequence()
            .filter(String::isNotEmpty)
            .map(String::toLong)
            .sumOf { id -> if (freshRanges.any { id in it }) 1L else 0L }
    }

    override fun part2(): Long {
        val rawRanges = input.first()

        return rawRanges
            .lineSequence()
            .map {
                val (left, right) = it.split('-').map(String::toLong)
                left..right
            }
            .mergeRanges()
            .sumOf { it.last - it.first + 1 }
    }
}

fun Sequence<LongRange>.mergeRanges() : List<LongRange> =
    sortedBy { it.first }.let { sortedRanges ->
        buildList {
            for (range in sortedRanges) {
                val previous = lastOrNull()
                if (previous == null || range.first > previous.last) add(range)
                else this[lastIndex] = previous.first..maxOf(previous.last, range.last)
            }
        }
    }