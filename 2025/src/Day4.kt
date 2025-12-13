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
        val grid = Grid(data().bufferedReader().readText())
        var sum = 0

        for ((y, row) in grid.stateMap.withIndex()) {
            for ((x) in row.withIndex()) {
                val point = Pair(x, y)
                if (!grid[point]) continue

                val blocked = dirs.map { dir ->
                    point.add(dir)
                }.count { point ->
                    grid.pointOnGrid(point) && grid[point]
                }

                if (blocked < 4) {
                    sum++
                }
            }
        }

        return sum
    }

    override fun part2(): Int {
        val grid = Grid(data().bufferedReader().readText())
        val queue = ArrayDeque<Point>()
        var sum = 0

        fun processCell(point: Point) {
            if (!grid[point]) return

            val blocked = dirs.map {dir ->
                point.add(dir)
            }.filter { point ->
                grid.pointOnGrid(point) && grid[point]
            }

            if (blocked.count() < 4) {
                sum++
                grid[point] = false
                queue.addAll(blocked)
            }
        }

        for ((y, row) in grid.stateMap.withIndex()) {
            for ((x) in row.withIndex()) {
                processCell(Pair(x,y))
                while (queue.isNotEmpty()) {
                    processCell(queue.removeFirst())
                }
            }
        }

        return sum
    }
}

typealias Point = Pair<Int, Int>

fun Point.add(p: Point): Point {
    return Point(this.first + p.first, this.second + p.second)
}

private class Grid(data: String) {
    val stateMap = data
        .lineSequence()
        .filterNot { it.isEmpty() }
        .map { line ->
            BooleanArray(line.length) { idx ->
                line[idx] == '@'
            }
        }
        .toList()
        .toTypedArray()
    val rowLen = stateMap[0].size // newlines are outside the grid
    val numRows = stateMap.size

    operator fun get(point: Point): Boolean {
        return stateMap[point.first][point.second]
    }

    operator fun set(point: Point, value: Boolean) {
        stateMap[point.first][point.second] = value
    }

    fun pointOnGrid(point: Point): Boolean {
        return point.first in 0..<rowLen && point.second in 0..<numRows
    }
}

//private class Grid(val data: String): Iterable<Triple<Char, Int, Point>> {
//    val rowLen = data.indexOf('\n') // newlines are outside the grid
//    val rawRowLen = rowLen + 1
//    val numRows = data.length / rawRowLen
//
//    operator fun get(idx: Int): Char {
//        return data[idx]
//    }
//
//    fun pointToIdx(point: Point): Int? {
//        if (!pointOnGrid(point)) return null
//        return point.second * rawRowLen + point.first
//    }
//
//    fun idxToPoint(idx: Int): Point? {
//        if (idx !in 0..<data.length) return null
//        return Point(idx % rawRowLen, idx / rawRowLen)
//    }
//
//    fun pointOnGrid(point: Point): Boolean {
//        return point.first in 0..<rowLen && point.second in 0..<numRows
//    }
//
//    override fun iterator(): Iterator<Triple<Char, Int, Point>> {
//        return object {
//            var currIdx = 0
//
//            operator fun hasNext() = currIdx < data.length - 1
//            operator fun next(): Triple<Char, Int, Point> {
//                val result = Triple(data[currIdx], currIdx, idxToPoint(currIdx)!!)
//                currIdx++
//
//                if (data[currIdx].isWhitespace()) currIdx++
//                return result
//            }
//        }.let {
//            object : Iterator<Triple<Char, Int, Point>> {
//                override fun hasNext() = it.hasNext()
//                override fun next() = it.next()
//            }
//        }
//    }
//}