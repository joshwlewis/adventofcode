package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	file, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	is, err := ReadInstructions(file)
	check(err)

	f := Ferry{D: 90}
	dist, err := f.Route(is)
	check(err)

	fmt.Println("Manhattan Distance: ", dist)
}

type Instruction struct {
	Action byte
	Value  int
}
type Instructions []Instruction

type Ferry struct {
	X int
	Y int
	D int
}

func ReadInstructions(r io.Reader) (Instructions, error) {
	scnr := bufio.NewScanner(r)
	is := Instructions{}
	for scnr.Scan() {
		text := strings.TrimSpace(scnr.Text())
		a := text[0]
		v, err := strconv.Atoi(text[1:])
		if err != nil {
			return is, err
		}
		is = append(is, Instruction{a, v})
	}
	err := scnr.Err()
	return is, err
}

func (i Instruction) String() string {
	return fmt.Sprintf("<%c %d>", i.Action, i.Value)
}

func (i Instruction) Change() (int, int, int, int, error) {
	switch i.Action {
	case byte('N'):
		return 0, i.Value, 0, 0, nil
	case byte('S'):
		return 0, -i.Value, 0, 0, nil
	case byte('E'):
		return i.Value, 0, 0, 0, nil
	case byte('W'):
		return -i.Value, 0, 0, 0, nil
	case byte('L'):
		return 0, 0, -i.Value, 0, nil
	case byte('R'):
		return 0, 0, i.Value, 0, nil
	case byte('F'):
		return 0, 0, 0, i.Value, nil
	}
	return 0, 0, 0, 0, fmt.Errorf("Invalid Action %c", i.Action)
}

func (f Ferry) Route(is Instructions) (int, error) {
	for _, i := range is {
		err := f.Move(i)
		if err != nil {
			return 0, err
		}
	}
	return int(math.Abs(float64(f.X)) + math.Abs(float64(f.Y))), nil
}

func (f *Ferry) Move(i Instruction) error {
	dx, dy, dd, fw, err := i.Change()
	if err != nil {
		return err
	}
	if fw != 0 {
		switch (f.D + 2880) % 360 {
		case 0:
			dy += fw
		case 90:
			dx += fw
		case 180:
			dy -= fw
		case 270:
			dx -= fw
		default:
			return fmt.Errorf("what angle is this?? %d, %d", f.D, f.D % 360)
		}
	}
	f.X += dx
	f.Y += dy
	f.D += dd
	return nil
}

func check(err error) {
	if err != nil {
		log.Fatalf("Unexpected error: %+v", err)
	}
}
