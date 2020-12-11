package main

import (
	"bytes"
	"testing"
)

var example = `
	L.LL.LL.LL
	LLLLLLL.LL
	L.L.L..L..
	LLLL.LL.LL
	L.LL.LL.LL
	L.LLLLL.LL
	..L.L.....
	LLLLLLLLLL
	L.LLLLLL.L
	L.LLLLL.LL
`

func TestStabilize(t *testing.T) {
	rdr := bytes.NewReader([]byte(example))
	room, err := ReadRoom(rdr)
	if err != nil {
		t.Fatalf("Unexpected error: %v\n", err)
	}

	newR := room.Stabilize()
	got := newR.Occupancies()
	want := 37
	if got != want {
		t.Fatalf("wanted %d, got %d", want, got)
	}
}
