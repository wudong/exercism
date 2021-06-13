package raindrops

import (
	"strconv"
	"strings"
)

func Convert(input int) string {
	var sb strings.Builder
	notMatch := true
	if input%3 == 0 {
		sb.WriteString("Pling")
		notMatch = false
	}
	
	if input%5 == 0 {
		sb.WriteString("Plang")
		notMatch = false
	}

	if input%7 == 0 {
		sb.WriteString("Plong")
		notMatch = false
	}

	if notMatch {
		sb.WriteString(strconv.Itoa(input))
	}

	return sb.String()

}
