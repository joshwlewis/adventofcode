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
	dir, err := os.Getwd()
	check(err)
	f, err := os.Open(filepath.Join(dir, "input.txt"))
	check(err)

	tm, err := NewTreeMapFromReader(f)
	check(err)
	trees := tm.TreesOnPath(3, 1)

	fmt.Printf("Found %d trees on the path\n", trees)
	os.Exit(0)
}

func check(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

type TreeRow []bool
type TreeMap struct {
	Width int
	Rows []TreeRow
}

func NewTreeMapFromReader(f io.Reader) (TreeMap, error) {
	scnr := bufio.NewScanner(f)
	tm := TreeMap{}
	for scnr.Scan() {
		line := scnr.Text()
		if tm.Width == 0 {
			tm.Width = len([]rune(line))
		}
		row := TreeRow{}
		for _, val := range scnr.Text() {
			row = append(row, val == '#')
		}
		tm.Rows = append(tm.Rows, row)
	}
	if err := scnr.Err(); err != nil {
		return tm, err
	}
	return tm, nil
}

func (tm *TreeMap) HasTree(x int, y int) bool {
	if y >= len(tm.Rows) {
		return false
	}
	vx := x % tm.Width
	return tm.Rows[y][vx]
}

func (tm *TreeMap) IsExit(y int) bool {
	return y == len(tm.Rows)
}

func (tm *TreeMap) TreesOnPath(dx int, dy int) int {
	var x, y, count int
	for {
		if tm.HasTree(x, y) {
			count++
		}
		if tm.IsExit(y) {
			break
		}
		x+=dx
		y+=dy
	}
	return count
}
