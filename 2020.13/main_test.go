package main

import "testing"

func TestContestTime(t *testing.T) {
	busIDs := []int{7, 13, 59, 31, 19}
	busOffs := []int{0, 1, 4, 6, 7}
	want := 1068781
	got := ContestTime(busIDs, busOffs)
	if want != got {
		t.Fatalf("Wanted %d, Got %d", want, got)
	}
}
