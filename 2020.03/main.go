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
	path := Path{3, 1}
	trees := tm.TreesOnPath(path)

	fmt.Printf("Found %d trees on the path %v\n", trees, path)

	paths := []Path{{1,1},{3,1},{5,1},{7,1},{1,2}}
	treeprod := tm.TreeProdOnPaths(paths)

	fmt.Printf("Tree product for paths %v\n", treeprod)
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
type Path struct {
	dx int
	dy int
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
	return y > len(tm.Rows)
}

func (tm *TreeMap) TreesOnPath(p Path) int {
	var x, y, count int
	for {
		if tm.IsExit(y) {
			break
		}
		if tm.HasTree(x, y) {
			count++
		}
		x+=p.dx
		y+=p.dy
	}
	return count
}

func (tm *TreeMap) TreeProdOnPaths(paths []Path) (int) {
	prod := 1
	for _, path := range paths {
		count := tm.TreesOnPath(path)
		prod *= count
	}
	return prod
}
