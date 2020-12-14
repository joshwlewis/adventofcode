package main

import (
	"testing"
)

func TestTwoPow(t *testing.T) {
	if TwoPow(0) != 1 {
		t.Errorf("Expected 2**0 to be 1, got %d", TwoPow(0))
	}
	if TwoPow(4) != 16 {
		t.Errorf("Expected 2**4 to be 16, got %d", TwoPow(4))
	}
}

func TestInstructionResult(t *testing.T) {
	i := Instruction{
		Mask: "1XXXX0X",
		Value: 11,
	}
	got := i.Result()
	want := 73
	if got != want {
		t.Errorf("wanted %d, got %d", want, got)
	}
}
