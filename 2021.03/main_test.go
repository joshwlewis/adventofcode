package main

import (
	"strings"
	"testing"
)

const sampleData = `00100
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
`

func TestGetPowerRates(t *testing.T) {
	input := strings.NewReader(sampleData)
	d,w := ReadDiagnostics(input);
	wantGam := 22
	wantEps := 9
	gotGam, gotEps := GetPowerRates(d,w)
	if gotGam != wantGam {
		t.Errorf("wanted gamma of %d, but got %d", wantGam, gotGam)
	}
	if gotEps != wantEps {
		t.Errorf("wanted epsilon of %d, but got %d", wantEps, gotEps)
	}
}

func TestGetLifeSupportRates(t *testing.T) {
	input := strings.NewReader(sampleData)
	d,w := ReadDiagnostics(input);
	wantO2 := 23
	wantCo2 := 10
	gotO2, gotCo2 := GetLifeSupportRates(d,w);
	if gotO2 != wantO2 {
		t.Errorf("wanted O2 of %d, but got %d", wantO2, gotO2)
	}
	if gotCo2 != wantCo2 {
		t.Errorf("wanted CO2 of %d, but got %d", wantCo2, gotCo2)
	}
}

