val left = Point(0, -1)
val right = Point(0, 1)
val above = Point(-1, 0)
val below = Point(1, 0)

class Day7(fileName: String) : DayTemplate<Long>(fileName) {
    override fun part1(): Long {
        val manifold = Manifold(data().bufferedReader().lineSequence())
        
        var splitterCount = 0L
        for ((y, line) in manifold.grid.withIndex()) {
            for ((x, ch) in line.withIndex()) {
                if (ch == '^') {
                    var p = Point(y - 1, x)
                    while (manifold.pointOnGrid(p)) {
                        if (manifold[p] == 'S') {
                            splitterCount++
                            break
                        } else if (manifold[p] == '^') break
                        else if (p.second == 0) break
                        else if (manifold[p.add(left)] == '^' || manifold[p.add(right)] == '^') {
                            splitterCount++
                            break
                        }
                        p = p.add(above)
                    }
                    // search upwards until hit 'S', top row, '^', searching for '^' 
                }
            }
        }
        return splitterCount
    }

    override fun part2(): Long {
        val manifold = Manifold(data().bufferedReader().lineSequence())
        val costs = manifold.grid.map { chArr -> LongArray(chArr.size) { 0L } }
        
        for ((y, line) in manifold.grid.withIndex()) {
            for ((x, ch) in line.withIndex()) {
                if (ch == 'S') {
                    val start = Point(y+2, x)
                    costs[start.first][start.second] = 1 // set initial num ways
                } else if (ch == '^') {
                    // take 1 right and 1 left, mark with cost to reach splitter
                    // travel down until hit another splitter, += that cost
                    var left = Point(y, x-1)
                    while (left.first < manifold.grid.lastIndex && manifold[left] != '^') {
                        left = left.add(below)
                    }
                    costs[left.first][left.second] += costs[y][x]

                    var right = Point(y, x+1)
                    while (right.first < manifold.grid.lastIndex && manifold[right] != '^') {
                        right = right.add(below)
                    }
                    costs[right.first][right.second] += costs[y][x]
                }
            }
        }
        return costs.last().sum()
    }
}

private class Manifold(data: Sequence<String>) {
    val grid = data
        .filterNot { it.isEmpty() }
        .map { it.toCharArray() }
        .toList()
        
    val rowLen = grid[0].size // newlines are outside the grid
    val numRows = grid.size

    operator fun get(point: Point): Char {
        return grid[point.first][point.second]
    }

    operator fun set(point: Point, value: Char) {
        grid[point.first][point.second] = value
    }

    fun pointOnGrid(point: Point): Boolean {
        return point.first in 0..<rowLen && point.second in 0..<numRows
    }
}