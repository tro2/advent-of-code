class Day6(fileName: String) : DayTemplate<Long>(fileName) {
    override fun part1(): Long {
        val rows = data()
            .bufferedReader()
            .lineSequence()
            .map { it.trim().split("\\s+".toRegex()) }
            .toList()

        val operands = rows.dropLast(1).map { it.map(String::toLong) }
        val operators = rows.last()

        var result = 0L

        for (idx in operators.indices) {
            val operands = operands.map { it[idx] }
            val value = when (operators[idx]) {
                "*" -> operands.reduce(Long::times)
                "+" -> operands.sum()
                else -> error("Unknown operator")
            }
            result += value
        }

        return result
    }

    override fun part2(): Long {
        val rows = data()
            .bufferedReader()
            .readLines()

        val operators = rows.last().trim().split("\\s+".toRegex())
        val operands = run {
            val operandGrid = rows.dropLast(1)
            operandGrid
                .first().indices // iterate over columns
                .map { idx -> // turn column into long, or null if all spaces
                    val column = operandGrid.map { it[idx] }
                    if (column.all(Char::isWhitespace)) {
                        return@map null
                    }
                    column
                        .filter(Char::isDigit)
                        .joinToString("")
                        .toLong()
                }
                .fold(mutableListOf<MutableList<Long>>()) { acc, value -> // split list of longs on nulls and group
                    when {
                        value == null -> acc.add(mutableListOf())
                        acc.isEmpty() -> acc.add(mutableListOf(value))
                        else -> acc.last().add(value)
                    }
                    acc
                }
        }
        
        return operators.zip(operands)
            .sumOf { (op, operands) ->
                when (op) {
                    "*" -> operands.reduce(Long::times)
                    "+" -> operands.sum()
                    else -> 0
                }
            }
    }
}