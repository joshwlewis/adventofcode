package main

import "testing"

func TestCalculate(t *testing.T) {
	tt := []struct {
		input string
		want  int
	}{{
		input: "2 * 3 + (4 * 5)",
		want:  46,
	}, {
		input: "5 + (8 * 3 + 9 + 3 * 4 * 3)",
		want:  1445,
	}, {
		input: "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
		want:  669060,
	}, {
		input: "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
		want:  23340,
	}}

	for _, test := range tt {
		_, got, err := Calculate(test.input)
		if err != nil {
			t.Fatalf("Didn't expect an error, but got %+v while processing %s", err, test.input)
		}
		if int(got) != test.want {
			t.Fatalf("Wanted %d, but got %d from %s", test.want, got, test.input)
		}
	}
}
