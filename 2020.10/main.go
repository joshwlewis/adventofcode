package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
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
	combs := js.ChainCombos()

	fmt.Printf("jolt chain ones:%d, threes:%d, prod:%d\n", ones, threes, ones*threes)
	fmt.Printf("jolt chain combos: %d\n", combs)
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

var comboMultipliers = []int{1, 1, 2, 4, 7}

func (js Joltages) ChainCombos() int {
	sum := 1
	for _, g := range js.ConsGroups() {
		mult := comboMultipliers[len(g)-1]
		sum *= mult
	}
	return sum
}

func (js Joltages) ConsGroups() (gjs []Joltages) {
	gjs = []Joltages{{js[0]}}
	for i := 1; i < len(js); i++ {
		if js[i]-js[i-1] != 1 {
			gjs = append(gjs, Joltages{})
		}
		gjs[len(gjs)-1] = append(gjs[len(gjs)-1], js[i])
	}
	return gjs
}

func (js Joltages) ChainDiffs() (ones int, threes int) {
	for i := 1; i < len(js); i++ {
		if js[i]-js[i-1] == 1 {
			ones++
		} else if js[i]-js[i-1] == 3 {
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
