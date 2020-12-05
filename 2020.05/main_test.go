package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestSeats(t *testing.T) {
	tests := []struct {
		seat string
		row  int
		col  int
		id   int
	}{
		{"FBFBBFFRLR", 44, 5, 357},
		{"FFFBBBFRRR", 14, 7, 119},
		{"BBFFBBFRLL", 102, 4, 820},
	}

	for _, tc := range tests {
		gotSeat, err := ParseSeat(tc.seat)
		if err != nil {
			t.Errorf("Unexpected error %v", err)
		}
		wantSeat := Seat{tc.row, tc.col}

		diff := cmp.Diff(wantSeat, gotSeat)
		if diff != "" {
			t.Errorf(diff)
		}

		gotID := gotSeat.ID()
		if gotID != tc.id {
			t.Errorf("Wanted seat ID %d, Got seat ID %d", tc.id, gotID)
		}
	}
}
