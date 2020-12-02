package main

import (
	"testing"
)

func TestDecodeAndValidate(t *testing.T) {
	tests := []struct {
		input string
		want  bool
	}{
		{"1-3 a: abcde", true},
		{"1-3 b: cdefg", false},
		{"2-9 c: ccccccccc", true},
	}

	for _, test := range tests {
		pass := NewPassFromText(test.input)
		got := pass.IsValid()
		if got != test.want {
			t.Fatalf("expected %v IsValid() to be %v; got %v", test.input, test.want, got)
		}
	}
}
