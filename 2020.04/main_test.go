package main

import (
	"strings"
	"testing"
)

var testdata = `
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
`

func TestCountValidIDs(t *testing.T) {
	rdr := strings.NewReader(testdata)
	want := 2
	got, err := CountValidIds(rdr)
	if err != nil {
		t.Errorf("Didn't expect this error %s\n", err)
	}
	if want != got {
		t.Errorf("Wanted %d, Got %d", want, got)
	}
}
