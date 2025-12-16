import kotlin.math.sqrt

class Day8(fileName: String) : DayTemplate<Long>(fileName) {
    override fun part1(): Long {
        val nodes = parseInput()

        val edges = nodes
            .flatMap { a ->
                nodes
                    .asSequence()
                    .filter { it != a }
                    .map { normalizeConnection(a, it) }
                    .toList()
            }
            .distinctBy { it }
            .sortedBy { it.euclidean() }
            .take(if (nodes.size == 20) 10 else 1000)

        // take first 10

        val circuits = DisjointSet(nodes)
        
        for ((a, b) in edges) {
            circuits.union(a, b)
        }


        return circuits.parent.keys
            .asSequence()
            .filter { circuits.parent[it] == it }
            .map { circuits.size[it]!!.toLong() }
            .sortedDescending()
            .take(3)
            .reduce { a, b -> a * b }
    }

    override fun part2(): Long {
        val nodes = parseInput()
        val edges = nodes
            .flatMap { a ->
                nodes
                    .asSequence()
                    .filter { it != a }
                    .map { normalizeConnection(a, it) }
                    .toList()
            }
            .distinctBy { it }
            .sortedBy { it.euclidean() }
        
        val circuits = DisjointSet(nodes)

        for ((a, b) in edges) {
            circuits.union(a, b)
            if (circuits.count() == 1) {
                return a.x * b.x
            }
        }
        
        return 0L // mistake, should not get here
    }
    
    fun parseInput(): List<Vec3> {
        return data()
            .bufferedReader()
            .lineSequence()
            .map {
                val (x, y, z) = it
                    .splitToSequence(',')
                    .map(String::toLong)
                    .toList()
                Vec3(x, y, z)
            }
            .toList()
    }
}

class DisjointSet(nodes: Iterable<Vec3>) {
    val parent = HashMap(nodes.associateWith { it })
    val size = HashMap(nodes.associateWith { 1 })
    
    fun find(node: Vec3): Vec3 {
        if (parent[node] != node)
            parent[node] = find(parent[node]!!)
        return parent[node]!!
    }
    
    fun union(a: Vec3, b: Vec3): Boolean {
        val parentA = find(a)
        val parentB = find(b)
        if (parentA == parentB) return false // nodes already in same tree
        
        // merge trees into the larger of the two
        if (size[parentA]!! < size[parentB]!!) {
            parent[parentA] = parentB
            size[parentB] = size[parentB]!! + size[parentA]!!
        } else {
            parent[parentB] = parentA
            size[parentA] = size[parentB]!! + size[parentA]!!
        }
        return true
    }
    
    fun count(): Int {
        return parent.keys.count { parent[it] == it }
    }
}

data class Vec3(val x: Long, val y: Long, val z: Long)

fun Vec3.euclidean(other: Vec3): Double {
    return subtract(other).let { (x, y, z) ->
        sqrt(x.toDouble() * x + y.toDouble() * y + z.toDouble() * z)
    }
}

fun Vec3.subtract(other: Vec3): Vec3 {
    return Vec3(x - other.x, y - other.y, z - other.z)
}

fun Vec3.equiv(other: Vec3): Boolean {
    return x == other.x && y == other.y && z == other.z
}

fun Vec3.compareTo(other: Vec3): Int = 
    compareValuesBy(this, other, Vec3::x, Vec3::y, Vec3::z)

fun normalizeConnection(a: Vec3, b: Vec3): Connection =
    if (a.compareTo(b) <= 0) Connection(a, b)
    else Connection(b,a)

data class Connection(val left: Vec3, val right: Vec3)

fun Connection.equiv(other: Connection): Boolean {
     return left.equiv(other.right) && right.equiv(other.left)
}

fun Connection.euclidean(): Double {
    return left.euclidean(right)
}