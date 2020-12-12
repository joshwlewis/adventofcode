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

	d := Driver{D: 90}
	dDist, err := Route(&d, is)
	check(err)
	fmt.Println("Driver Manhattan Distance: ", dDist)

	c := Chaser{WX: 10, WY: 1}
	cDist, err := Route(&c, is)
	check(err)

	fmt.Println("Chaser Manhattan Distance: ", cDist)
}

type Instruction struct {
	Action byte
	Value  int
}
type Instructions []Instruction

type Ferry interface {
	Dist() int
	Move(Instruction) error
}

type Driver struct {
	X int
	Y int
	D int
}

type Chaser struct {
	FX int
	FY int
    WX int
	WY int
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

func Route(f Ferry, is Instructions) (int, error) {
	for _, i := range is {
		err := f.Move(i)
		if err != nil {
			return 0, err
		}
	}
	return f.Dist(), nil
}

func (c Chaser) Dist() int {
	return int(math.Abs(float64(c.FX)) + math.Abs(float64(c.FY)))
}
func (c *Chaser) Move(i Instruction) error {
	dx, dy, dd, fw, err := i.Change()
	if err != nil {
		return err
	}
	c.WX, c.WY, err = Rotate(c.WX, c.WY, (dd+360) % 360)
	if err != nil {
		return err
	}
	c.WX+=dx
	c.WY+=dy
	for i := 0; i < fw; i++ {
		c.FX+=c.WX
		c.FY+=c.WY
	}
	return nil
}

func Rotate(x, y, ang int) (int, int, error) {
	switch ang {
	case 0:
		return x, y, nil
	case 90:
		return y, -x, nil
	case 180:
		return -x, -y, nil
	case 270:
		return -y, x, nil
	}
	return 0, 0, fmt.Errorf("What angle is this?? %d", ang)
}

func (d Driver) Dist() int {
	return int(math.Abs(float64(d.X)) + math.Abs(float64(d.Y)))
}
func (f *Driver) Move(i Instruction) error {
	dx, dy, dd, fw, err := i.Change()
	if err != nil {
		return err
	}
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
