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

	r.Searcher = AdjSearcher{}
	rAdjStable := r.Stabilize()
	fmt.Printf("Stabilized Occupancies By Adjacency: %d\n", rAdjStable.Occupancies())

	r.Searcher = VisSearcher{}
	rVisStable := r.Stabilize()
	fmt.Printf("Stabilized Occupancies By Visibility: %d\n", rVisStable.Occupancies())
}

type Tile rune
type Tiles []Tile
type Room struct {
	Rows []Tiles
	Searcher Searcher
}
type AdjSearcher struct {}
type VisSearcher struct {}
type Searcher interface {
	Nearby(Room, int, int) Tiles
	Occupied(Room, int, int) bool
	Vacant(Room, int, int) bool
}

var empty = Tile('L')
var occupied = Tile('#')
var floor = Tile('.')
var wall = Tile('+')

func ReadRoom(r io.Reader) (Room, error) {
	scnr := bufio.NewScanner(r)
	var y int
	rows := []Tiles{}
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		if line == "" {
			continue
		}
		row := Tiles{}
		for _, c := range line {
			row = append(row, Tile(c))
		}
		rows = append(rows, row)
		y++
	}
	err := scnr.Err()
	return Room{Rows: rows}, err
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
	newRoom := Room{Searcher: r.Searcher}
	for y, row := range r.Rows {
		newRow := Tiles{}
		for x, tile := range row {
			if tile == floor {
				newRow = append(newRow, floor)
				continue
			}
			if tile == empty && r.Searcher.Vacant(r, x, y) {
				newRow = append(newRow, occupied)
				continue
			}
			if tile == occupied && r.Searcher.Occupied(r, x, y) {
				newRow = append(newRow, empty)
				continue
			}
			newRow = append(newRow, tile)
		}
		newRoom.Rows = append(newRoom.Rows, newRow)
	}
	return newRoom
}

func (r Room) Eq(otherR Room) bool {
	if len(r.Rows) != len(otherR.Rows) {
		return false
	}
	for y, row := range r.Rows {
		if len(row) != len(otherR.Rows[y]) {
			return false
		}
		for x, tile := range row {
			if tile != otherR.Rows[y][x] {
				return false
			}
		}
	}
	return true
}

func vacantArea(r Room, s Searcher, x, y int) bool {
	for _, t := range s.Nearby(r, x, y) {
		if t == occupied {
			return false
		}
	}
	return true
}
func (rs AdjSearcher) Vacant(r Room, x, y int) bool {
	return vacantArea(r, rs, x, y)
}

func (rs VisSearcher) Vacant(r Room, x, y int) bool {
	return vacantArea(r, rs, x, y)
}

func occupiedArea(r Room, s Searcher, maxCount, x, y int) bool {
	occupiedCount := 0
	for _, t := range s.Nearby(r, x, y) {
		if t == occupied {
			occupiedCount++
			if occupiedCount >= maxCount {
				return true
			}
		}
	}
	return false
}

func (rs AdjSearcher) Occupied(r Room, x, y int) bool {
	return occupiedArea(r, rs, 4, x, y)
}

func (rs VisSearcher) Occupied(r Room, x, y int) bool {
	return occupiedArea(r, rs, 5, x, y)
}

func (rs AdjSearcher) Nearby(r Room, x, y int) Tiles {
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

func (rs VisSearcher) Nearby(r Room, x, y int) (tiles Tiles) {
	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			if dx == 0 && dy == 0 {
				continue
			}
			for i, j:= x+dx, y+dy; true; i, j = i+dx, j+dy {
				t := r.Tile(i, j)
				if t != floor {
					tiles = append(tiles, t)
					break 
				}
			}
		}
	}
	return
}

func (r Room) Tile(x, y int) Tile {
	if y >= 0 && x >= 0 && y < len(r.Rows) && x < len(r.Rows[y]) {
		return r.Rows[y][x]
	}
	return wall
}

func (r Room) Occupancies() (vs int) {
	for _, row := range r.Rows {
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
