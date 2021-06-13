package twelve

import (
	"fmt"
	"strings"
)

var numberStrMap = map[int]struct{
	count string
	stuff string} {
		1: {count:"first", stuff:"a Partridge in a Pear Tree"},
		2: {count:"second",stuff:"two Turtle Doves"},
		3: {count:"third",stuff:"three French Hens"},
		4: {count:"fourth", stuff:"four Calling Birds"},
		5: {count:"fifth", stuff:"five Gold Rings"},
		6: {count:"sixth", stuff:"six Geese-a-Laying"},
		7: {count:"seventh", stuff:"seven Swans-a-Swimming"},
		8: {count:"eighth", stuff:"eight Maids-a-Milking"},
		9: {count:"ninth", stuff:"nine Ladies Dancing"},
		10: {count:"tenth", stuff:"ten Lords-a-Leaping"},
		11: {count:"eleventh", stuff:"eleven Pipers Piping"},
		12: {count:"twelfth", stuff:"twelve Drummers Drumming"},
	}

func Song() string {
	sb := strings.Builder{}
	for i:=1;i<12;i++{
		sb.WriteString(Verse(i))
		sb.WriteRune('\n')
	}
	sb.WriteString(Verse(12))
	return sb.String()
}


func Verse(input int) string {
	ss := numberStrMap[input]
	sb := strings.Builder{}
	firstPart := fmt.Sprintf("On the %s day of Christmas my true love gave to me: ", ss.count)
	sb.WriteString(firstPart)
	for i:=input; i>1; i-- {
		sb.WriteString(numberStrMap[i].stuff)
		sb.WriteString(", ")
	}
	if input > 1 {
		sb.WriteString("and ")
	}

	sb.WriteString(numberStrMap[1].stuff)
	sb.WriteRune('.')

	return sb.String()
}