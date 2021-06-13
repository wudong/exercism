package grains

import (
	"fmt"
)

const TotalSquare = 64

func Total() uint64 {
	var memory [TotalSquare]uint64
	total:=uint64(0)

	for i:=0;i<TotalSquare;i++{
		if i==0 {
			memory[i] = 1
		} else {
			memory[i] = memory[i-1] * 2
		}
		total += memory[i]
	}
	return total
}

func Square(input int) (uint64, error) {
	if input <= 0 || input > TotalSquare {
		return 0, fmt.Errorf("invalid input: %d", input)
	}

	result :=uint64(1)
	for i:=1;i<=input; i++{
		if i==1 {
			result = 1
		}else {
			result = result * 2
		}
	}
	return result,nil
}