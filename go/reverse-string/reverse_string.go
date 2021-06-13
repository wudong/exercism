package reverse

import "strings"

func Reverse(input string) string {
	sb := strings.Builder{}
	r:=[]rune(input)
	length := len(r)
	for i:=length-1; i>=0; i-- {
		u := r[i]
		sb.WriteRune(rune(u))
	}
	return sb.String()
}
