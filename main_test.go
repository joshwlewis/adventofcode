package main

import "testing"

func TestExampleFindInvalid(t *testing.T) {
	d := Data{35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576}
	got, err := d.FindInvalidNumber(5)
	if err != nil {
		t.Errorf("Unexpected error: %+v/n", err)
	}
	want := 127
	if got != want {
		t.Errorf("Wanted %d, but got %d for %+v\n", want, got, d)
	}
}
