package main

import (
	"strings"
	"testing"
)

func TestGetRates(t *testing.T) {
	input := strings.NewReader(`00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
`)
	wantGam := int64(22)
	wantEps := int64(9)
	gotGam, gotEps := GetRates(input)
	if gotGam != wantGam {
		t.Errorf("wanted gamma of %d, but got %d", wantGam, gotGam)
	}
	if gotEps != wantEps {
		t.Errorf("wanted epsilon of %d, but got %d", wantEps, gotEps)
	}
}
