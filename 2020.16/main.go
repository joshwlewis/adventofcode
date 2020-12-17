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

	rules, myTicket, tickets, err := readData(f)
	check(err)

	errRate := calcErrorRate(tickets, rules)
	fmt.Println("error rate", errRate)

	validTix := filterTickets(tickets, rules)
	tKey := decipherKey(validTix, rules)
	prod := myTicket.prod(tKey)
	fmt.Println("prod", prod)
}

type ticket []int
type tickets []ticket
type ruleRange [2]int
type ruleRanges []ruleRange
type rule struct {
	name   string
	ranges [2]ruleRange
}
type rules []rule
type translationKey map[int]string

func calcErrorRate(ts tickets, rs rules) (rate int) {
	rngs := consolidateRanges(rs)
	for _, t := range ts {
		for _, v := range t {
			if !validVal(v, rngs) {
				rate += v
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
	for i := 1; i <= 1000; i++ {
		valid := validVal(i, rrs)
		if start == 0 && valid {
			start = i
			continue
		}
		if start != 0 && !valid {
			rngs = append(rngs, ruleRange{start, i - 1})
			start = 0
		}
	}
	return
}

func validVal(v int, rs ruleRanges) bool {
	for _, r := range rs {
		if v >= r[0] && v <= r[1] {
			return true
		}
	}
	return false
}

func validTicket(t ticket, rs ruleRanges) bool {
	for _, v := range t {
		if !validVal(v, rs) {
			return false
		}
	}
	return true
}

func filterTickets(tix tickets, rls rules) (tks tickets) {
	rngs := consolidateRanges(rls)
	for _, t := range tix {
		if validTicket(t, rngs) {
			tks = append(tks, t)
		}
	}
	return tks
}

func decipherKey(ts tickets, rls rules) translationKey {
	pKey := partialKey(ts, rls)
	tKey := make(translationKey)
	A:
	for {
		for k, cols := range pKey {
			var lastMatch, matchCount int
			for _, col := range cols {
				if tKey[col] == "" {
					lastMatch = col
					matchCount++
				}
			}
			if matchCount == 1 {
				tKey[lastMatch] = k
			}
		}
		if len(pKey) == len(tKey) {
			break A
		}
	}
	return tKey
}

func partialKey(ts tickets, rls rules) map[string][]int {
	trans := make(map[string][]int)
	for _, rl := range rls {
	I:
		for i := 0; i < len(rls); i++ {
			for _, t := range ts {
				if !validVal(t[i], rl.ranges[:]) {
					continue I
				}
			}
			trans[rl.name] = append(trans[rl.name], i)
		}
	}
	return trans
}

func readData(r io.Reader) (rs rules, t ticket, ts tickets, err error) {
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
			r.name = parts[0]
			_, err = fmt.Fscanf(strings.NewReader(parts[1]), "%d-%d or %d-%d", &r.ranges[0][0], &r.ranges[0][1], &r.ranges[1][0], &r.ranges[1][1])
			if err != nil {
				return
			}
			rs = append(rs, r)
		case 1:
			vs := strings.Split(line, ",")
			for _, v := range vs {
				var val int
				val, err = strconv.Atoi(v)
				if err != nil {
					return
				}
				t = append(t, val)
			}

		case 2:
			vs := strings.Split(line, ",")
			nt := ticket{}
			for _, v := range vs {
				var val int
				val, err = strconv.Atoi(v)
				if err != nil {
					return
				}
				nt = append(nt, val)
			}
			ts = append(ts, nt)
		}
	}
	err = scnr.Err()
	return
}

func (t ticket) prod(key translationKey) int {
	prod := 1
	for col, name := range key {
		var suf string
		fmt.Sscanf(name, "departure %s", &suf)
		if suf != "" {
			prod *= t[col]
		}
	}
	return prod
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected error: %+v", err)
	}
}
