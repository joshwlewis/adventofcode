package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	rules, tickets, err := readData(f)
	check(err)
	fmt.Println("rules", rules)

	errRate := calcErrorRate(tickets, rules)
	fmt.Println("error rate", errRate)
}

type ticket []int
type tickets []ticket
type ruleRange [2]int
type ruleRanges []ruleRange
type rule struct {
	key  string
	ranges [2]ruleRange
}
type rules []rule

func calcErrorRate(ts tickets, rs rules) (rate int) {
	rngs := consolidateRanges(rs)
	for _, t := range ts {
		for _, v := range t {
			if !validVal(v, rngs) {
				rate += int(v)
			}
		}
	}
	return
}

func consolidateRanges(rls rules) (rngs []ruleRange) {
	var start int
	var rrs []ruleRange
	for _, rl := range rls {
		rrs = append(rrs, rl.ranges[0], rl.ranges[1])
	}
	for i:=1; i <= 1000; i++ {
		valid := validVal(i, rrs)
		if start == 0 && valid {
			start = i
			continue
		}
		if start != 0 && !valid {
			rngs = append(rngs, ruleRange{start, i-1})
			start = 0
		}
	}
	return
}
func validVal(v int, rs ruleRanges) (bool) {
	for _, r := range rs {
		if v >= r[0] && v <= r[1] {
			return true
		}
	}
	return false
}

func readData(r io.Reader) (rs rules, ts tickets, err error) {
	scnr := bufio.NewScanner(r)
	var group int
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		switch line {
		case "":
			continue
		case "your ticket:", "nearby tickets:":
			group++
			continue
		}
		switch group {
		case 0:
			r := rule{}
			parts := strings.Split(line, ": ")
			r.key = parts[0]
			_, err = fmt.Fscanf(strings.NewReader(parts[1]), "%d-%d or %d-%d", &r.ranges[0][0], &r.ranges[0][1], &r.ranges[1][0], &r.ranges[1][1])
			if err != nil {
				return
			}
			rs = append(rs, r)
		case 2:
			vs := strings.Split(line, ",")
			t := ticket{}
			for _, v := range vs {
				var val int
				val, err = strconv.Atoi(v)
				if err != nil {
					return
				}
				t = append(t, val)
			}
			ts = append(ts, t)
		}
	}
	err = scnr.Err()
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected error: %+v", err)
	}
}
