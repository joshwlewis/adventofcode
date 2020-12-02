package main

import (
	"testing"
)

func TestDecodeAndValidate(t *testing.T) {
	tests := []struct {
		input string
		sled  bool
		toboggan bool
	}{
		{"1-3 a: abcde", true, true},
		{"1-3 b: cdefg", false, false},
		{"2-9 c: ccccccccc", true, false},
	}

	for _, test := range tests {
		pass := NewPassFromText(test.input)
		sled := pass.IsValidSled()
		if sled != test.sled {
			t.Fatalf("expected %v IsValidSled() to be %v; got %v", test.input, test.sled, sled)
		}
		toboggan := pass.IsValidTobbggan()
		if toboggan != test.toboggan {
			t.Fatalf("expected %v IsValidTobbggan() to be %v; got %v", test.input, test.toboggan, toboggan)
		}
	}
}
