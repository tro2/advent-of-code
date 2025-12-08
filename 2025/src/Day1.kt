import kotlin.math.absoluteValue

const val STARTING_POS = 50
const val DIAL_LEN = 100

class Day1 : DayTemplate<Int> {
    constructor(fileName: String) : super(fileName)

    override fun part1(): Int {
        var pos = STARTING_POS
        var countZeros = 0

        for (line in data().bufferedReader().lineSequence()) {
            val turnLeft = line[0] == 'L'
            val distance = line.substring(1..<line.length).toInt()

            pos = if (turnLeft) { // if starts with L, rotate left with wrap around
                (pos - distance).mod(DIAL_LEN)
            } else { // if starts with R, rotate right with wrap around (mod)
                (pos + distance) % DIAL_LEN
            }
            if (pos == 0) countZeros++
        }

        return countZeros
    }

    override fun part2(): Int {
        var pos = STARTING_POS
        var countZeros = 0

        for (line in data().bufferedReader().lineSequence()) {
            val turnLeft = line[0] == 'L'
            val distance = line.substring(1..<line.length).toInt()

            pos = if (turnLeft) { // if starts with L, rotate left with wrap around
                if (pos > 0 && pos - distance <= 0) countZeros++
                countZeros += (pos - distance).absoluteValue / DIAL_LEN
                (pos - distance).mod(DIAL_LEN)
            } else { // if starts with R, rotate right with wrap around (mod)
                countZeros += (pos + distance) / DIAL_LEN
                (pos + distance) % DIAL_LEN
            }

        }

        return countZeros
    }
}