class Day4(fileName: String) : DayTemplate<Int>(fileName) {
    val dirs = listOf(
        Point(-1, -1),  // Northwest
        Point(0, -1),   // North
        Point(1, -1),   // Northeast
        Point(1, 0),    // East
        Point(1, 1),    // Southeast
        Point(0, 1),    // South
        Point(-1, 1),   // Southwest
        Point(-1, 0)    // West
    )

    override fun part1(): Int {
        val data = Grid(data().bufferedReader().readText())
        var sum = 0

        for ((ch, idx) in data) {
            val currPoint = data.idxToPoint(idx) ?: continue
            if (ch == '@') {
                val count = dirs.count { dir ->
                    val offset = currPoint.add(dir)
                    if (!data.pointOnGrid(offset)) return@count false

                    data[data.pointToIdx(offset) ?: return@count false] == '@'
                }

                if (count < 4) {
                    sum++
                }
            }
        }

        return sum
    }

    override fun part2(): Int {
        return 0
    }
}

typealias Point = Pair<Int, Int>

fun Point.add(p: Point): Point {
    return Point(this.first + p.first, this.second + p.second)
}

private class Grid(val data: String): Iterable<Pair<Char, Int>> {
    val rowLen = data.indexOf('\n') // newlines are outside the grid
    val rawRowLen = rowLen + 1
    val numRows = data.length / rawRowLen

    operator fun get(idx: Int): Char {
        return data[idx]
    }

    fun pointToIdx(point: Point): Int? {
        if (!pointOnGrid(point)) return null
        return point.second * rawRowLen + point.first
    }

    fun idxToPoint(idx: Int): Point? {
        if (idx !in 0..<data.length) return null
        return Point(idx % rawRowLen, idx / rawRowLen)
    }
    
    fun pointOnGrid(point: Point): Boolean {
        return point.first in 0..<rowLen && point.second in 0..<numRows
    }

    override fun iterator(): Iterator<Pair<Char, Int>> {
        return object {
            var currIdx = 0

            operator fun hasNext() = currIdx < data.length - 1
            operator fun next(): Pair<Char, Int> {
                val result = Pair(data[currIdx], currIdx)
                currIdx++
                if (data[currIdx].isWhitespace()) currIdx++
                return result
            }
        }.let {
            object : Iterator<Pair<Char, Int>> {
                override fun hasNext() = it.hasNext()
                override fun next() = it.next()
            }
        }
    }
}