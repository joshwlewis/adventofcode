package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	js, err := ParseJoltages(f)
	check(err)

	js = js.WithTerminals()
	sort.Sort(js)

	ones, threes := js.ChainDiffs()
	check(err)

	fmt.Printf("jolt chain ones:%d, threes:%d, prod:%d\n", ones, threes, ones*threes)

	arrs := js.ChainArrangements()

	fmt.Printf("jolt chain arrangements: %d\n", arrs)
}

type Joltages []int

func ParseJoltages(r io.Reader) (Joltages, error) {
	scnr := bufio.NewScanner(r)
	js := Joltages{}
	for scnr.Scan() {
		j, err := strconv.Atoi(strings.TrimSpace(scnr.Text()))
		if err != nil {
			return js, err
		}
		js = append(js, j)
	}
	err := scnr.Err()
	return js, err
}
func (js Joltages) WithTerminals() Joltages {
	max := 0
	for _, v := range js {
		if v > max {
			max = v
		}
	}
	js = append(js, 0, max+3)
	return js
}

func (js Joltages) Len() int           { return len(js) }
func (js Joltages) Swap(i, j int)      { js[i], js[j] = js[j], js[i] }
func (js Joltages) Less(i, j int) bool { return js[i] < js[j] }

func (js Joltages) ChainArrangements() int {
	var p2, p7 float64
	for i := 1; i < len(js)-1; i++ {
		if i >= 3 && js[i+1]-js[i-3] == 4 {
			p7++
			p2 -= 2
			continue
		}
		if js[i+1]-js[i-1] == 2 {
			p2++
		}
	}
	return int(math.Pow(2.0, p2) * math.Pow(7.0, p7))
}

func (js Joltages) ChainDiffs() (ones int, threes int) {
	for i := 1; i < len(js); i++ {
		if js[i] - js[i-1] == 1 {
			ones++
		} else if js[i] - js[i-1] == 3 {
			threes++
		}
	}
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("Boom: %+v\n", err)
	}
}
