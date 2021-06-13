package letter

import "fmt"

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func combineFreqMap(m1 *FreqMap, m2 FreqMap) {
	for k, v := range m2 {
		(*m1)[k] +=v
	}
}

func ConcurrentFrequency(strings []string) FreqMap {
	size:=len(strings)
	chans := make(chan FreqMap, size)

	for _, s:= range strings{
		go func(str string) {
			frequency := Frequency(str)
			fmt.Printf("Compute frequence of : %d for string:\n %s\n", len(frequency), str)
			chans <- frequency
		}(s)
	}

	mm := FreqMap{}
	for i:=0; i<size; i++ {
		m:= <-chans
		fmt.Printf("Getting frequence of : %d\n", len(m))
		combineFreqMap(&mm, m)
	}

	return mm
}