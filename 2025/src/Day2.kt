class Day2(fileName: String) : DayTemplate<Long>(fileName) {

    override fun part1(): Long {
        return data()
            .bufferedReader()
            .readText()
            .split(',')
            .sumOf { chunk ->
                val (l, r) = chunk.split('-').map { s -> s.toLong() }

                (l..r).sumOf { i ->
                    val str = i.toString()
                    if (str.length % 2 != 0) return@sumOf 0
                    val half = str.length / 2
                    val left = str.substring(0..<half)
                    val right = str.substring(half..<str.length)
                    if (left == right) i else 0
                }
            }
    }

    override fun part2(): Long {
        return data()
            .bufferedReader()
            .readText()
            .split(',')
            .sumOf { chunk ->
                val (l, r) = chunk.split('-').map { s -> s.toLong() }

                (l..r).sumOf { i ->
                    val str = i.toString()
                    if (str.length == 1) return@sumOf 0
                    val half = (str.length + 1) / 2

                    val yes = (1..half).reversed().any { len ->
                        if (str.length % len != 0) return@any false
                        // check every exclusive substring of length len to see if they all eq
                        val chunks = str.chunked(len)
                        val first = chunks[0]
                        chunks.all { it == first }
                    }
                    if (yes) i else 0
                }
            }
    }
}