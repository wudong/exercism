package luhn

import (
	"fmt"
	"strings"
)

func Valid(input string) bool {

	//check if all are digit and remove space
	digits, err := preProcess(input)
	if err {
		return false
	}

	if len(digits) <=1 {
		return false
	}
	// start from right, double every second digit,
	// sum all the digits.
	// to see if it can be divided by 10
	toDouble:=false
	sum:=0
	for i:=len(digits)-1; i>=0; i-- {
		o:=int(digits[i]-'0')
		v:=o
		if toDouble {
			v=o*2
			if v > 9 {
				v = v - 9
			}
		}
		sum+=v
		fmt.Printf("sum so far %d with %d\n", sum, o)
		toDouble = !toDouble
	}
	return sum % 10 == 0
}

func preProcess(input string) (string, bool) {
	//remove space.
	sb := strings.Builder{}
	for _, v := range input {
		if v != ' ' && v >= '0' && v <= '9' {
			sb.WriteRune(v)
		} else if v == ' '{
			continue
		} else { //invalid
			return "", true//invalid
		}
	}
	result := sb.String()
	fmt.Printf("preprocessing %s to %s\n", input, result)
	return result, false
}