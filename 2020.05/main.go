package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)
    
	maxID, err := MaxSeatID(f)
	check(err)

	fmt.Println("Max Seat ID:", maxID)
}

func check(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

func MaxSeatID(r io.Reader) (int, error) {
	var max int
	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		row, col, err := ParseSeat(scnr.Text())
		if err != nil {
			return max, err
		}
		id := SeatID(row, col)
		if id > max {
			max = id
		}
	}
	if scnr.Err() != nil {
		return 0, scnr.Err()
	}
	return max, nil
}

func ParseSeat(seat string) (row, col int, err error) {
	row, err = decodePartition(seat[0:7], 'F', 'B')
	if err != nil {
		return 0, 0, err
	}
	col, err = decodePartition(seat[7:10], 'L', 'R')
	if err != nil {
		return 0, 0, err
	}
	return
}

func SeatID(row, col int) int {
	return (row * 8) + col
}

func decodePartition(chars string, lowerChar, upperChar rune) (int, error) {
	min, max := 1, 1
	exp := len(chars)
	for exp != 0 {
		max *= 2
		exp -= 1
	}
	for _, c := range chars {
		adj := (max - min + 1) / 2
		switch c {
		case upperChar:
			min += adj
		case lowerChar:
			max = max - adj
		default:
			return 0, fmt.Errorf("Expected %v or %v, got %v in %+v", lowerChar, upperChar, c, chars)
		}
	}
	if min != max {
		return 0, fmt.Errorf("Couldn't partition %+v. Min: %d, Max: %d", chars, min, max)
	}
	return min - 1, nil
}
