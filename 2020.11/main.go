package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	r, err := ReadRoom(f)
	check(err)

	r = r.Stabilize()
	fmt.Printf("Stabilized Occupancies: %d\n", r.Occupancies())
}

type Tile rune
type Tiles []Tile
type Room []Tiles

var empty = Tile('L')
var occupied = Tile('#')
var floor = Tile('.')
var wall = Tile('+')

func ReadRoom(r io.Reader) (Room, error) {
	scnr := bufio.NewScanner(r)
	var y int
	room := Room{}
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		if line == "" {
			continue
		}
		row := Tiles{}
		for _, c := range line {
			row = append(row, Tile(c))
		}
		room = append(room, row)
		y++
	}
	err := scnr.Err()
	return room, err
}

func (r Room) Stabilize() Room {
	iRoom := r
	for {
		jRoom := iRoom.Advance()
		if iRoom.Eq(jRoom) {
			return iRoom
		}
		iRoom = jRoom
	}
}

func (r Room) Advance() Room {
	newRoom := Room{}
	for y, row := range r {
		newRow := Tiles{}
		for x, tile := range row {
			if tile == floor {
				newRow = append(newRow, floor)
				continue
			}
			if tile == empty {
				if r.VacantArea(x, y) {
					newRow = append(newRow, occupied)
					continue
				}
				newRow = append(newRow, empty)
				continue
			}
			if tile == occupied {
				if r.CrowdedArea(x, y) {
					newRow = append(newRow, empty)
					continue
				}
				newRow = append(newRow, occupied)
				continue
			}
		}
		newRoom = append(newRoom, newRow)
	}
	return newRoom
}

func (r Room) Eq(otherR Room) bool {
	if len(r) != len(otherR) {
		return false
	}
	for y, row := range r {
		if len(row) != len(otherR[y]) {
			return false
		}
		for x, tile := range row {
			if tile != otherR[y][x] {
				return false
			}
		}
	}
	return true
}

func (r Room) VacantArea(x, y int) bool {
	for _, t := range r.AdjacentTiles(x, y) {
		if t == occupied {
			return false
		}
	}
	return true
}

func (r Room) CrowdedArea(x, y int) bool {
	occupiedCount := 0
	for _, t := range r.AdjacentTiles(x, y) {
		if t == occupied {
			occupiedCount++
			if occupiedCount >= 4 {
				break
			}
		}
	}
	return occupiedCount >= 4
}

func (r Room) PopulatedArea(x, y int) bool {
	occupiedCount := 0
	for _, t := range r.VisibleTiles(x, y) {
		if t == occupied {
			occupiedCount++
			if occupiedCount >= 5 {
				break
			}
		}
	}
	return occupiedCount >= 4
}


func (r Room) AdjacentTiles(x, y int) Tiles {
	var tiles Tiles
	for i := x - 1; i <= x+1; i++ {
		for j := y - 1; j <= y+1; j++ {
			if i == x && j == y {
				continue
			}
			tiles = append(tiles, r.Tile(i, j))
		}
	}
	return tiles
}

func (r Room) VisibleTile(x, y, dx, dy int) (Tile) {
	for i := x+dx; ; i+=dx {
		for j := y+dy; ; j+=dy {
			t := r.Tile(i, j)
			if t != floor {
				return t
			}
		}
	}
}
func (r Room) VisibleTiles(x, y int) (tiles Tiles) {
	for dx := x - 1; dx <= x+1; dx++ {
		for dy := y - 1; dy <= y+1; dy++ {
			tiles = append(tiles, r.VisibleTile(x, y, dx, dy))
		}
	}
	return
}

func (r Room) Tile(x, y int) Tile {
	if y >= 0 && x >= 0 && y < len(r) && x < len(r[y]) {
		return r[y][x]
	}
	return wall
}

func (r Room) Occupancies() (vs int) {
	for _, row := range r {
		for _, tile := range row {
			if tile == occupied {
				vs++
			}
		}
	}
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("Unexpected Error: %+v\n", err)
	}
}
