package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"sort"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	seats, err := ParseSeats(f)
	check(err)

	seats.Sort()

	maxID := seats[len(seats)-1].ID()
	missingID, err := seats.Missing()
	check(err)

	fmt.Println("Max Seat ID:", maxID)
	fmt.Println("Missing Seat ID:", missingID)
}

func check(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

type Seats []Seat
type Seat struct {
	Row int
	Col int
}

func ParseSeats(r io.Reader) (seats Seats, err error) {
	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		seat, err := ParseSeat(scnr.Text())
		if err != nil {
			return seats, err
		}
		seats = append(seats, seat)
	}
	err = scnr.Err()
	return
}

func (ss Seats) Sort() {
	sort.Slice(ss, func(i, j int) bool {
		return ss[i].ID() < ss[j].ID()
	})
}

func (ss Seats) Missing() (int, error) {
	for i, s := range ss {
		if s.Row == 0 || s.Row == 127 {
			continue
		}
		if ss[i+1].ID() > s.ID()+1 {
			return s.ID() + 1, nil
		}
	}
	return 0, fmt.Errorf("Did not find missing seat")
}

func ParseSeat(seat string) (Seat, error) {
	row, err := decodePartition(seat[0:7], 'F', 'B')
	if err != nil {
		return Seat{}, err
	}
	col, err := decodePartition(seat[7:10], 'L', 'R')
	if err != nil {
		return Seat{}, err
	}
	return Seat{row, col}, nil
}

func (s Seat) ID() int {
	return (s.Row * 8) + s.Col
}

func decodePartition(chars string, lowerChar, upperChar rune) (int, error) {
	min, max := 1, 1
	exp := len(chars)
	for exp != 0 {
		max *= 2
		exp--
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
