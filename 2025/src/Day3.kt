class Day3(fileName: String) : DayTemplate<Long>(fileName) {
    override fun part1() : Long {
        var sum = 0L

        for (line in data().bufferedReader().lines()) {
            val firstIdx = findGreatestIdx(0, line.length - 2, line)
            val secondIdx = findGreatestIdx(firstIdx + 1, line.length - 1, line)
            sum += (line[firstIdx] - '0') * 10 + (line[secondIdx] - '0')
        }

        return sum
    }

    fun findGreatestIdx(startIdx: Int, endIdx: Int, str: String): Int {
        var max = 0
        var index = 0
        for (idx in startIdx..endIdx) {
            val value = str[idx] - '0'
            if (value > max) {
                max = value
                index = idx
            }
        }
        return index
    }

    override fun part2() : Long {
        var sum = 0L

        for (line in data().bufferedReader().lines()) {
            var innerSum = 0L
            var prevIdx = 0
            for (i in (1..12).reversed()) {
                val idx = findGreatestIdx(prevIdx, line.length - i, line)
                prevIdx = idx + 1
                innerSum = innerSum * 10L + (line[idx] - '0')
            }

            sum += innerSum
        }

        return sum
    }
}