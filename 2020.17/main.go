package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"strings"
)

var input = `
######.#
#.###.#.
###.....
#.####..
##.#.###
.######.
###.####
######.#
`

func main() {
	plane, err := ReadPlane(strings.NewReader(input))
	check(err)

	space := BuildSpace(plane)
	space = space.Boot(6)
	active := space.ActiveCount()
	fmt.Println("Active in space after boot:", active)
}

type Axis map[int]bool
type Plane map[int]Axis
type Space map[int]Plane

func BuildSpace(p Plane) (Space) {
	s := make(Space)
	s[0] = p
	return s
}

func (space Space) Boot(cycles int) (Space) {
	for i := 1; i <= cycles; i++ {
		space = space.Cycle()
	}
	return space
}

func (space Space) Cycle() (Space) {
	newSpace := make(Space)
	xB, yB, zB := space.Bounds()
	for x := xB[0]-1; x <= xB[1]+1; x++ {
		for y := yB[0]-1; y <= yB[1]+1; y++ {
			for z := zB[0]-1; z <= zB[1]+1; z++ {
				active := space.Active(x, y, z)
				activeNearby := space.ActiveNearby(x,y,z)
				if active && (activeNearby == 2 || activeNearby == 3) {
					newSpace.Activate(x, y, z)
				}
				if !active && (activeNearby == 3) {
					newSpace.Activate(x, y, z)
				}
			}
		}
	}
	return newSpace
}

func (s Space) ActiveNearby(x int, y int, z int) (sum int) {
	for ix := x-1; ix <= x+1; ix++ {
		for iy := y-1; iy <= y+1; iy++ {
			for iz := z-1; iz <= z+1; iz++ {
				if ix == x && iy == y && iz == z {
					continue
				}
				if s.Active(ix, iy, iz) {
					sum++
				}
			}
		}
	}
	return
}

type Bounds [2]int

func (space Space) Bounds() (Bounds, Bounds, Bounds) {
	xB, yB, zB := Bounds{}, Bounds{}, Bounds{}
	for z, plane := range space {
		zB.Grow(z)
		for y, axis := range plane {
			yB.Grow(y)
			for x := range axis {
				xB.Grow(x)
			}
		}
	}
	return xB, yB, zB
}

func (b *Bounds)Grow(v int) {
	if v < b[0] {
		b[0] = v
	}
	if v > b[1] {
		b[1] = v
	}
}
func (space Space) Activate(x int, y int, z int) {
	plane, pok := space[z]
	if !pok {
		plane = make(Plane)
	}
	axis, aok := plane[y]
	if !aok {
		axis = make(Axis)
	}
	axis[x] = true
	plane[y] = axis
	space[z] = plane
}

func (s Space) Active(x int, y int, z int) (bool) {
	plane, pok := s[z]
	if !pok {
		return false
	}
	axis, aok := plane[y]
	if !aok {
		return false
	}
	return axis[x]
}

func (space Space) ActiveCount() (sum int) {
	for _, plane := range space {
		for _, axis := range plane {
			for _, active := range axis {
				if active {
					sum++
				}
			}
		}
	}
	return
}

func (space Space) String() (rep string) {
	xB, yB, zB := space.Bounds()
	for z := zB[0]; z <= zB[1]; z++ {
		rep += fmt.Sprintf("z=%d\n", z)
		for y := yB[0]; y <= yB[1]; y++ {
			rep += fmt.Sprint("[")
			for x := xB[0]; x <= xB[1]; x++ {
				if space.Active(x, y, z) {
					rep+="#"
				} else {
					rep+=" "
				}
			}
			rep += fmt.Sprint("]")
			rep += fmt.Sprintln()
		}
	}
	return
}

func ReadPlane(r io.Reader) (Plane, error) {
	scnr := bufio.NewScanner(r)
	plane := make(Plane)
	var y int
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		if line == "" {
			continue
		}
		axis := make(Axis)
		for x, c := range line {
			if c == '#' {
				axis[x] = true
			}
		}
		plane[y] = axis
		y++
	}
	err := scnr.Err()
	return plane, err
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected error: %v", err)
	}
}
