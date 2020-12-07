package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	br, err := ParseBagRules(f)
	check(err)

	outerBagSum := br.SumOuterBagsFor("shiny gold")
	fmt.Println("Outer bag option count for shiny gold:", outerBagSum)
}

type BagRules map[string][]string

var outerBagPattern = regexp.MustCompile(`^([a-z|\s]+) bags contain (.+)\.$`)
var innerBagPattern = regexp.MustCompile(`^(\d+) ([a-z|\s]+) bags?$`)

func ParseBagRules(r io.Reader) (br BagRules, err error) {
	br = BagRules{}
	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		oMatches := outerBagPattern.FindStringSubmatch(line)
		oBag := oMatches[1]
		var iBags []string
		if oMatches[2] != "" {
			for _, iString := range strings.Split(oMatches[2], ",") {
				iString := strings.TrimSpace(iString)
				iMatches := innerBagPattern.FindStringSubmatch(iString)
				if len(iMatches) == 3 {
					iBags = append(iBags, iMatches[2])
				}
			}
		}
		br[oBag] = iBags
	}
	err = scnr.Err()
	return
}

func (brs BagRules) SumOuterBagsFor(tBag string) int {
	cache := make(map[string]*bool)
	var sum int
	for oBag := range brs {
		if brs.CanContain(oBag, tBag, cache) {
			sum++
		}
	}
	return sum
}

var t = true
var f = false
func (brs BagRules) CanContain(outer string, inner string, cache map[string]*bool) bool {
	if cache[outer] != nil {
		return *cache[outer]
	}
	for _, iBag := range brs[outer] {
		if iBag == inner {
			cache[outer] = &t
			return true
		}
		if brs.CanContain(iBag, inner, cache) {
			cache[outer] = &t
			return true
		}
	}
	cache[outer] = &f
	return false
}

func check(err error) {
	if err != nil {
		log.Fatalf("Unexpected error: %+v\n", err)
	}
}
