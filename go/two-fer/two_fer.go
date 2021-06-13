package twofer

import "fmt"

func ShareWith(name string) string {

	greet := name
	if len(name) == 0 {
		greet = "you"
	}

	return fmt.Sprintf("One for %s, one for me.", greet)
}
