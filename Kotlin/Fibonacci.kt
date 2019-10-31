fun fibo(n: Int): Int {
    return if (n < 2) n else fibo(n - 1) + fibo(n - 2)
}

fun main(args: Array<String>) {
    println("Hello Hacktoberfest")
    print("Digite o número de n: ")
    val n = readLine()!!.toInt()
    for (i in 0..n)
        print(" ${fibo(i)}")
}
