package main

import (
	"strings"
	"testing"
)

func TestParseAndDetect(t *testing.T) {
	rdr := strings.NewReader("#..#\n.#.#\n#.#.")
	tm, err := NewTreeMapFromReader(rdr)
	if err != nil {
		t.Errorf("error reading test data: %v\n", err)
	}

	tests := []struct {
		x int
		y int
		t bool
	}{
		{0, 0, true},
		{0, 1, false},
		{0, 2, true},
		{3, 0, true},
		{2, 0, false},
		{2, 1, false},
		{3, 2, false},
		{4, 0, true},
		{5, 2, false},
		{7, 1, true},
	}

	for _, tc := range tests {
		tree := tm.HasTree(tc.x, tc.y)
		if tree != tc.t {
			t.Fatalf("Expected HasTree() to be %v, but got %v at %v,%v", tc.t, tree, tc.x, tc.y)
		}
	}
}
