package clock

import (
	"fmt"
)

type Clock struct {
	hour int
	minute int
}

func New(hour, minute int) Clock {
	hour += minute / 60

	minute = minute % 60
	if minute < 0 {
		minute +=60
		hour=hour-1
	}

	hour = hour % 24
	if hour < 0 {
		hour +=24
	}

	return Clock{
		hour: hour,
		minute: minute,
	}
}

func (l Clock) String() string {
	return fmt.Sprintf("%02d:%02d", l.hour, l.minute)
}

func (l Clock) Add(minutes int) Clock {
	return New(l.hour, l.minute + minutes)
}

func (l Clock) Subtract(minutes int) Clock {
	return New(l.hour, l.minute - minutes)
}
