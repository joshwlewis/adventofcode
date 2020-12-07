package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	br, err := ParseBagRules(f)
	check(err)

	outerBagSum := br.OuterSum("shiny gold")
	innerBagSum := br.InnerSum("shiny gold")

	fmt.Println("Outer bag count for shiny gold:", outerBagSum)
	fmt.Println("Inner bag sum for shiny gold:", innerBagSum)
}

type BagRules map[string]map[string]int

var outerBagPattern = regexp.MustCompile(`^([a-z|\s]+) bags contain (.+)\.$`)
var innerBagPattern = regexp.MustCompile(`^(\d+) ([a-z|\s]+) bags?$`)

func ParseBagRules(r io.Reader) (br BagRules, err error) {
	br = BagRules{}
	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		oMatches := outerBagPattern.FindStringSubmatch(line)
		oBag := oMatches[1]
		iBags := make(map[string]int)
		if oMatches[2] != "" {
			for _, iString := range strings.Split(oMatches[2], ",") {
				iString := strings.TrimSpace(iString)
				iMatches := innerBagPattern.FindStringSubmatch(iString)
				if len(iMatches) == 3 {
					iBagCount, err := strconv.Atoi(iMatches[1])
					if err != nil {
						return br, err
					}
					iBags[iMatches[2]] = iBagCount
				}
			}
		}
		br[oBag] = iBags
	}
	err = scnr.Err()
	return
}

func (brs BagRules) OuterSum(tBag string) (sum int) {
	for oBag := range brs {
		if brs.CanContain(oBag, tBag) {
			sum++
		}
	}
	return
}

func (brs BagRules) CanContain(outer string, inner string) bool {
	for iBag := range brs[outer] {
		if iBag == inner {
			return true
		}
		if brs.CanContain(iBag, inner) {
			return true
		}
	}
	return false
}

func (brs BagRules) InnerSum(outer string) (sum int) {
	for iBagColor, iBagCount := range brs[outer] {
		sum += iBagCount * (brs.InnerSum(iBagColor) + 1)
	}
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("Unexpected error: %+v\n", err)
	}
}
