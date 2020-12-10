package main

import (
	"sort"
	"testing"
)

var example1 = Joltages{28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3}.WithTerminals()

func TestExample1Diffs(t *testing.T) {
	sort.Sort(example1)
	gotOnes, gotThrees := example1.ChainDiffs()
	wantOnes, wantThrees := 22, 10
	if gotOnes != wantOnes || gotThrees != wantThrees {
		t.Errorf("Wanted %d/%d, Got %d/%d", wantOnes, wantThrees, gotOnes, gotThrees)
	}
}

func TestExample1Arrangements(t *testing.T) {
	sort.Sort(example1)
	got := example1.ChainArrangements()
	want := 19208
	if got != want {
		t.Errorf("Wanted %d, Got %d", want, got)
	}
}
