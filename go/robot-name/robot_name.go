package robotname

import (
	"math/rand"
	"strings"
)

type Robot struct {
	name string
}

func (r *Robot) Name() (string, error) {

	if r.name == "" {
		sb:=strings.Builder{}
		sb.WriteRune(randomChar())
		sb.WriteRune(randomChar())
		sb.WriteRune(randomNum())
		sb.WriteRune(randomNum())
		sb.WriteRune(randomNum())
		r.name = sb.String()
	}

	return r.name, nil
}

func randomChar() rune {
	size := int('Z'-'A')
	intn := rand.Intn(size) + 'A'
	return int32(intn)
}

func randomNum() rune {
	size := int('9'-'0')
	intn := rand.Intn(size) + '0'
	return int32(intn)
}

func (r *Robot) Reset()  {
	r.name = ""
}