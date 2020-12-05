package main

import "testing"

func TestSeats(t *testing.T) {
	tests := []struct{
		seat string
		row int
		col int
		id int
	}{
		{ "FBFBBFFRLR", 44, 5, 357 },
		{ "FFFBBBFRRR", 14, 7, 119 },
		{ "BBFFBBFRLL", 102, 4, 820 },
	}

	for _, tc := range tests {
		gotRow, gotCol, err := ParseSeat(tc.seat)
		if err != nil {
			t.Errorf("Unexpected error %v", err)
		}
		if gotRow != tc.row || gotCol != tc.col {
			t.Errorf("Wanted position %d,%d, Got position %d,%d", tc.row, tc.col, gotRow, gotCol)
		}
		
		gotID := SeatID(gotRow, gotCol)
		if gotID != tc.id {
			t.Errorf("Wanted seat ID %d, Got seat ID %d", tc.id, gotID)
		}
	}
}
