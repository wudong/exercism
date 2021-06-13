package isogram

import "strings"

func IsIsogram(input string) bool {
	mm:= make(map[int32]bool)
	for _, c := range strings.ToLower(input) {
		if c== ' ' || c== '-' {
			continue
		}
		if _, ok:=mm[c]; ok {
			return false
		}

		mm[c]=true
	}
	return true
}
