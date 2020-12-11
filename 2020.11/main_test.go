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

func testStabilized(s Searcher, want int, t *testing.T) {
	rdr := bytes.NewReader([]byte(example))
	room, err := ReadRoom(rdr)
	if err != nil {
		t.Fatalf("Unexpected error: %v\n", err)
	}

	room.Searcher = s
	newR := room.Stabilize()
	got := newR.Occupancies()
	if got != want {
		t.Fatalf("wanted %d, got %d", want, got)
	}
}

func TestStabilized(t *testing.T) {
	testStabilized(AdjSearcher{}, 37, t)
	testStabilized(VisSearcher{}, 26, t)
}
