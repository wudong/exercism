package diffsquares

func SquareOfSum(n int) int {
	result:=0
	for i:=1; i<=n; i++ {
		result += i
	}
	return result * result
}

func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}

func SumOfSquares(n int) int {
	result:=0
	for i:=1; i<=n; i++ {
		result += i*i
	}
	return result
}
