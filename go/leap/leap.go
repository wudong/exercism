package leap

func IsLeapYear(year int) bool {
     divBy4 := (year % 4 == 0)
     divBy100 := (year % 100 == 0)
     divBy400 := (year % 400 == 0)
     return divBy4 && ( !divBy100  || divBy100 && divBy400 )
}