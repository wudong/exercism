package hamming

import "fmt"

func Distance(a, b string) (int, error) {
     if len(a) != len(b) {     
     	return 0, fmt.Errorf("Size is different")
     }

     count:=0
     for i:=0; i<len(a); i++ {
         if a[i]!=b[i] {
	    count = count +1 
	 }
     }
     return count, nil
}
