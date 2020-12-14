package main

import (
	"fmt"
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

func TestInstructionGetValue(t *testing.T) {
	i := Instruction{
		Mask:  "1XXXX0X",
		Value: 11,
	}
	got := i.GetValue()
	want := 73
	if got != want {
		t.Errorf("wanted %d, got %d", want, got)
	}
}

func TestInstructionGetAddresses(t *testing.T) {
	i := Instruction{
		Mask:    "X1001X",
		Address: 42,
	}
	gots := i.GetAddresses()
	wants := []int{26, 27, 58, 59}
	fmt.Println("Gots", gots)
	if len(gots) != len(wants) {
		t.Errorf("Wanted len %d, got len %d", len(wants), len(gots))
	}
	for i, want := range wants {
		if gots[i] != want {
			t.Errorf("wanted %d, got %d", want, gots[i])
		}
	}
}
