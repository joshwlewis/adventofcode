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
	diagnostics, width := ReadDiagnostics(f);
	gamma, epsilon := GetPowerRates(diagnostics, width)
	p_rating := gamma * epsilon
	fmt.Printf("POWER --> Gamma: %d, Epsilon: %d, Rating: %d\n", gamma, epsilon, p_rating)
	o2, co2 := GetLifeSupportRates(diagnostics, width)
	ls_rating := o2 * co2;
	fmt.Printf("LIFE SUPPORT --> O2: %d, CO2: %d, Rating: %d\n", o2, co2, ls_rating);
}

func ReadDiagnostics(input io.Reader) (diagnostics []int, width int) {
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		entry := scanner.Text()
		num, err := strconv.ParseInt(entry, 2, 32)
		if err != nil {
			panic(fmt.Sprintf("could not parse number %s", err))
		}
		if width == 0 {
			width = len(entry)
		}
		diagnostics = append(diagnostics, int(num))
	}
	return diagnostics, width
}

func GetPowerRates(diagnostics []int, width int) (gamma int, epsilon int) {
	return getCommons(diagnostics, 0, width, width)
}

func GetLifeSupportRates(diagnostics []int, width int) (o2 int, co2 int) {
	o2 = findDiagnostic(diagnostics, width, 0);
	co2 = findDiagnostic(diagnostics, width, 1);
	return o2, co2
}

func getCommons(diagnostics []int, start, count, width int) (most, least int) {
	sum := make([]int, count);
	for _, num := range diagnostics {
		for i := start; i < start+count; i++ {
			sum[i-start] += (num >> (width-i-1)) & 1
		}
	}
	for i, n := range sum {
		if n*2 >= len(diagnostics) {
			sum[i] = 1
		} else {
			sum[i] = 0
		}
	}
	for _, n := range sum {
		most = (most << 1) + int(n)
	}
	least = ^most & ((1 << count) - 1)
	return most, least;
}

func findDiagnostic(diagnostics []int, width, mode int) (int) {
	group := []int{}
	last_group := diagnostics
	for pos := 0; pos < width; pos++ {
		o2filter, co2filter := getCommons(last_group, pos, 1, width)
		var filter int;
		if mode == 0 {
			filter = o2filter;
		} else {
			filter = co2filter;
		}
		for _, n := range last_group {
			if (n >> (width-pos-1)) & 1 == filter {
				group = append(group, n)
			}
		}
		if len(group) == 1 {
			return group[0];
		}
		last_group = group;
		group = []int{};
	}
	return 0;
}
