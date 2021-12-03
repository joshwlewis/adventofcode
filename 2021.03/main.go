package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
)

func main() {
	wd, _ := os.Getwd()
	f, err := os.Open(filepath.Join(wd, "input.txt"))
	if err != nil {
		panic(fmt.Sprintf("could not open input.txt: %s", err))
	}
	defer f.Close()
	gamma, epsilon := GetRates(f)
	power := gamma * epsilon
	fmt.Printf("Gamma: %d, Epsilon: %d, Power: %d\n", gamma, epsilon, power)
}

func GetRates(input io.Reader) (gamma int64, epsilon int64) {
	var sum []int64
	entries := int64(0)
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		entries++
		entry := scanner.Text()
		num, err := strconv.ParseInt(entry, 2, 64)
		if err != nil {
			panic(fmt.Sprintf("could not parse number %s", err))
		}
		if len(sum) == 0 {
			sum = make([]int64, len(entry))
		}

		for i := 0; i < len(sum); i++ {
			sum[i] += (num >> (len(sum)-i-1)) & 1
		}
	}
	for i, n := range sum {
		if n < entries/2 {
			sum[i] = 0
		}
		if n > entries/2 {
			sum[i] = 1
		}
	}
	gamma = 0
	for _, n := range sum {
		gamma = (gamma << 1) + int64(n)
	}
	epsilon = ^gamma & ((1 << len(sum)) - 1)
	return gamma, epsilon
}
